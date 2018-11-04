use time;
use std::sync::{Arc, Mutex};

// temp var for test
#[allow(dead_code)]
pub const STANDARD_EPOCH: u64 = 1514736000_000u64;

// machine id's bit
#[allow(dead_code)]
const WORKER_ID_BITS: u8 = 10;
#[allow(dead_code)]
const SEQUENCE_ID_BITS: u8 = 12;
#[allow(dead_code)]
const TIMESTAMP_ID_BITS: u8 = 42;

// shift 
const WORKER_ID_SHIFT : u8 =12;
const TIMESTAMP_LEFT_SHIFT: u8 = 22;

// mask
#[allow(dead_code)]
const SEQUENCE_MASK : u16 = 0xFFF;

#[derive(Debug, Default)]
pub struct SnowFlakeId {
    // system begin running time, micro-second
    standard_epoch: u64,
    worker_id: u16,
    sequence: u16,
    last_timestamp: u64,
}

#[allow(dead_code)]
impl SnowFlakeId {
    pub fn new(worker_id: u16, standard: u64) -> Self {
        SnowFlakeId {
            standard_epoch: standard,
            worker_id : worker_id,
            sequence : 0,
            last_timestamp: 0,
        }
    }

    pub fn new_multi_thread(worker_id: u16, standard: u64) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SnowFlakeId {
            standard_epoch: standard,
            worker_id : worker_id,
            sequence : 0,
            last_timestamp: 0,
        }))
    }

    pub fn generate_id(&mut self) -> Result<u64, String> {
        let mut curr_timestamp = SnowFlakeId::curr_time_millisec();

        if curr_timestamp < self.last_timestamp {
            return  Err(format!("Clock moved backwards.  Refusing to generate id for {} milliseconds", self.last_timestamp));
        }

        if curr_timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            if self.sequence == 0 {
                if curr_timestamp == self.last_timestamp {
                    curr_timestamp = self.wait_for_next_milli_sec();
                }
            }
        } else {
            self.sequence = 0u16;
        }

        self.last_timestamp = curr_timestamp;
        let uid: u64 = (self.last_timestamp - self.standard_epoch) << TIMESTAMP_LEFT_SHIFT |
            (self.worker_id as u64) << WORKER_ID_SHIFT |
            (self.sequence as u64);

        Ok(uid)
    }

    pub fn wait_for_next_milli_sec(&self) -> u64 {
        let mut curr_timestamp = SnowFlakeId::curr_time_millisec();

        while self.last_timestamp >= curr_timestamp {
            curr_timestamp = SnowFlakeId::curr_time_millisec();
        }

        curr_timestamp
    }

    pub fn curr_time_millisec() -> u64 {
        let time_spec = time::get_time();
        let mut macro_sec = (time_spec.sec as u64) * 1000u64;
        macro_sec += (time_spec.nsec as u64) / 1000_000u64;
        macro_sec
    }
}

#[allow(unused_imports)]
mod test {
    use crate::snowflake::{self, SnowFlakeId};
    use std::thread;
    use std::time::Instant;

    #[test]
    fn loop_test(){
        let mut id_gen = SnowFlakeId::new(1, snowflake::STANDARD_EPOCH);
        println!("{:?}",&id_gen);
        let now = Instant::now();
        for _ in 1..1000 {
            let t  = &mut id_gen;
            assert!(t.generate_id().is_ok());
        }
        let elapsed = now.elapsed();
        println!("single thread generate 1000 ids cost {}.{:09} s",elapsed.as_secs(), elapsed.subsec_nanos());
    }

    #[test]
    fn multi_thread(){
        let id_gen = SnowFlakeId::new_multi_thread(2, snowflake::STANDARD_EPOCH);
        let mut ths = Vec::new();
        for i in 1 .. 10{
            let t = id_gen.clone();
            ths.push(thread::spawn(move || {
                let now = Instant::now();
                for _ in 1..1000 {
                    let mut gen = t.lock().unwrap();
                    let id = gen.generate_id();
                    assert!(id.is_ok());
                    //println!("{:?}",id.unwrap());
                }
                let elapsed = now.elapsed();
                println!("multi thread:[{}] generate 1000 ids cost {}.{:09} s", i, elapsed.as_secs(), elapsed.subsec_nanos());
            }));
        }

        for t in ths {
            t.join();
        }

    }
}
