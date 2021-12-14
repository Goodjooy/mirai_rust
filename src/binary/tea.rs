use std::io::Cursor;

use std::io::Read;
use std::io::Write;
use std::num::Wrapping;
use std::ops::Range;

use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use rand::RngCore;

const SUM_TABLE: [u32; 0x10] = [
    0x9e3779b9, 0x3c6ef372, 0xdaa66d2b, 0x78dde6e4, 0x1715609d, 0xb54cda56, 0x5384540f, 0xf1bbcdc8,
    0x8ff34781, 0x2e2ac13a, 0xcc623af3, 0x6a99b4ac, 0x08d12e65, 0xa708a81e, 0x454021d7, 0xe3779b90,
];

pub struct Tea([u32; 4]);

fn xor(src: &[u8; 8], rth: &[u8; 8]) -> [u8; 8] {
    [
        src[0] ^ rth[0],
        src[1] ^ rth[1],
        src[2] ^ rth[2],
        src[3] ^ rth[3],
        src[4] ^ rth[4],
        src[5] ^ rth[5],
        src[6] ^ rth[6],
        src[7] ^ rth[7],
    ]
}

fn into_u64(src: [u8; 8]) -> u64 {
    (src[0] as u64) << 56
        | (src[1] as u64) << 48
        | (src[2] as u64) << 40
        | (src[3] as u64) << 32
        | (src[4] as u64) << 24
        | (src[5] as u64) << 16
        | (src[6] as u64) << 8
        | (src[7] as u64)
}

fn from_u64(src: u64) -> [u8; 8] {
    [
        (src << 56) as u8,
        (src << 48) as u8,
        (src << 40) as u8,
        (src << 32) as u8,
        (src << 24) as u8,
        (src << 16) as u8,
        (src << 8) as u8,
        (src) as u8,
    ]
}

fn copy(dst: &mut [u8], src: &[u8]) -> Option<()> {
    if dst.len() >= src.len() {
        for i in 0..src.len() {
            let d = unsafe { src.get_unchecked(i) };
            let t = unsafe { dst.get_unchecked_mut(i) };
            *t = *d;
        }
        Some(())
    } else {
        None
    }
}

fn rand_u32() -> u32 {
    rand::random()
}

impl Tea {
    /// Encrypt tea 加密  
    /// http://bbs.chinaunix.net/thread-583468-1-1.html  
    /// 感谢xichen大佬对TEA的解释  
    /// [参考](https://github.com/Mrs4s/MiraiGo/blob/master/binary/tea.go)
    pub fn encrypt(&self, src: Vec<u8>) -> Option<Vec<u8>> {
        let src_size = src.len();
        let fill = 10 - (src_size + 1) % 8;
        let total_size = fill + src_size + 7;

        let mut dst = vec![0u8; total_size];
        dst[0] = ((fill - 3) as u8) | 0xF8;
        rand::thread_rng().fill_bytes(&mut dst[1..fill + 1]);

        copy(&mut dst[fill..], &src);

        let mut dst = Cursor::new(dst);
        let mut res_dst = Vec::with_capacity(total_size);
        //tr 为上次加密结果， to 为上次原文
        let (mut tr, mut to, mut buff) = (0, 0, 0);
        for _idx in (0..total_size).step_by(8) {
            let data = dst.read_u64::<BigEndian>().expect("Read data Failure");
            buff = data ^ tr;
            tr = self.encode(buff);
            tr = tr ^ to;
            to = buff;

            res_dst
                .write_u64::<BigEndian>(tr)
                .expect("Write data Failure");
        }

        Some(res_dst)
    }

    pub fn decrypt(&self, data: Vec<u8>) -> Option<Vec<u8>> {
        let data_size = data.len();
        if data.len() < 16 || data.len() % 8 != 0 {
            return None;
        } else {
            let mut src = Cursor::new(data);
            let mut dsc = Vec::with_capacity(data_size);
            let (mut v1, mut v2, mut holder) = (0u64, 0u64, 0u64);

            for _idx in (0..data_size).step_by(8) {
                v1 = src.read_u64::<BigEndian>().expect("Read data Failure");
                v2 ^= v1;
                v2 = self.decode(v2);
                dsc.write_u64::<BigEndian>(v2 ^ holder)
                    .expect("Write data Failure");
                holder = v1;
            }
            let datarange = ((dsc[0] & 7) + 3) as usize..data_size - 7;
            Some(Vec::from_iter(dsc[datarange].into_iter().map(|d| *d)))
        }
    }
}

impl Tea {
    fn encode(&self, src: u64) -> u64 {
        let (mut v0, mut v1) = ((src >> 32) as u32, src as u32);
        let [t0, t1, t2, t3] = &self.0;
        let mut v0 = Wrapping(v0);
        let mut v1 = Wrapping(v1);
        let t0 = Wrapping(*t0);
        let t1 = Wrapping(*t1);
        let t2 = Wrapping(*t2);
        let t3 = Wrapping(*t3);
        for v in SUM_TABLE {
            let v = Wrapping(v);
            v0 += (v1 + v) ^ ((v1 << 4) + t0) ^ ((v1 >> 5) + t1);
            v1 += (v0 + v) ^ ((v0 << 4) + t2) ^ ((v0 >> 5) + t3);
        }

        (v0.0 as u64) << 32 | (v1.0 as u64)
    }

    fn decode(&self, src: u64) -> u64 {
        let (mut v0, mut v1) = ((src >> 32) as u32, src as u32);
        let [t0, t1, t2, t3] = &self.0;
        let mut v0 = Wrapping(v0);
        let mut v1 = Wrapping(v1);
        let t0 = Wrapping(*t0);
        let t1 = Wrapping(*t1);
        let t2 = Wrapping(*t2);
        let t3 = Wrapping(*t3);
        for v in SUM_TABLE.into_iter().rev() {
            let v = Wrapping(v);
            v1 -= (v0 + v) ^ ((v0 << 4) + t2) ^ ((v0 >> 5) + t3);
            v0 -= (v1 + v) ^ ((v1 << 4) + t0) ^ ((v1 >> 5) + t1);
        }
        (v0.0 as u64) << 32 | (v1.0 as u64)
    }
}

impl Tea {
    pub fn new(key: &[u8]) -> Self {
        let mut rd = Cursor::new(key.into_iter().map(|d| *d).collect::<Vec<_>>());
        Self([
            rd.read_u32::<BigEndian>().unwrap(),
            rd.read_u32::<BigEndian>().unwrap(),
            rd.read_u32::<BigEndian>().unwrap(),
            rd.read_u32::<BigEndian>().unwrap(),
        ])
    }
}

#[cfg(test)]
mod test {
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;
    #[test]
    fn test_encrypt() {
        let key = "0123456789ABCDEF".as_bytes();
        let tea = Tea::new(key);
        let src = "MiraiGO Here".bytes().collect::<Vec<_>>();
        let res = tea.encrypt(src);

        assert_ne!(res, None);

        let res = res.unwrap();
        let dres = tea.decrypt(res).unwrap();
        assert_eq!("MiraiGO Here".bytes().collect::<Vec<_>>(), dres);
    }

    #[test]
    fn test_decrypt() {
        let tres = [
            0xb7, 0xb2, 0xe5, 0x2a, 0xf7, 0xf5, 0xb1, 0xfb, 0xf3, 0x7f, 0xc3, 0xd5, 0x54, 0x6a,
            0xc7, 0x56, 0x9a, 0xec, 0xd0, 0x1b, 0xba, 0xcf, 0x09, 0xbf,
        ];
        let key = "0123456789ABCDEF".as_bytes();
        let tea = Tea::new(key);
        let dres = tea.decrypt(tres.into_iter().collect()).unwrap();
        assert_eq!("MiraiGO Here".bytes().collect::<Vec<_>>(), dres);
    }

    #[test]
    fn random_data_test() {
        let key = "0123456789ABCDEF".as_bytes();
        let tea = Tea::new(key);

        for i in 0..5{
            println!("Doing Num {} test", i);
            let data: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(rand::random::<u8>() as usize)
                .map(char::from)
                .collect();
            println!("test String is {}", data);

            let ec = tea
                .encrypt(data.clone().bytes().into_iter().collect())
                .unwrap();

            //println!("ecrptyed is : {:?}", ec);

            let de = tea.decrypt(ec).unwrap();

            println!("decrptyed is   {}",  String::from_utf8_lossy(&de));

            assert_eq!(String::from_utf8_lossy(&de).to_string(), data);
        }
    }
}
