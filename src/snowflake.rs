use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// temp var for test
#[allow(dead_code)]
pub const STANDARD_EPOCH: i64 = 1514736000_000i64;

// machine id's bit
#[allow(dead_code)]
const WORKER_ID_BITS: u8 = 10;
#[allow(dead_code)]
const SEQUENCE_ID_BITS: u8 = 12;
#[allow(dead_code)]
const TIMESTAMP_ID_BITS: u8 = 42;

// shift
const WORKER_ID_SHIFT: u8 = 12;
const TIMESTAMP_LEFT_SHIFT: u8 = 22;

// mask
#[allow(dead_code)]
const SEQUENCE_MASK: u16 = 0xFFF;

#[derive(Debug, Default)]
pub struct IdStorage {
    pub sequence: u16,
    pub worker_id: u16,
    pub last_timestamp: i64,
}

impl IdStorage {
    pub fn gen_id(&self, curr_timestamp: i64, standard_epoch: i64) -> i64 {
        let uid: i64 = (curr_timestamp - standard_epoch) << TIMESTAMP_LEFT_SHIFT
            | (self.worker_id as i64) << WORKER_ID_SHIFT
            | (self.sequence as i64);
        uid
    }
}

#[derive(Debug, Default)]
pub struct SnowFlakeId {
    // system begin running time, micro-second
    standard_epoch: i64,
    id_cache_index: usize,
    id_caches: Vec<Box<IdStorage>>,
}

#[allow(dead_code)]
impl SnowFlakeId {
    pub fn new(worker_id: u16, standard: i64) -> Self {
        if worker_id > 511u16 {
            panic!("worker id should less than 512");
        }

        let id_caches = vec![
            Box::new(IdStorage {
                sequence: 0u16,
                worker_id: worker_id,
                last_timestamp: 0i64,
            }),
            Box::new(IdStorage {
                sequence: 0u16,
                worker_id: worker_id + 512,
                last_timestamp: 0i64,
            }),
        ];

        SnowFlakeId {
            standard_epoch: standard,
            id_cache_index: 0,
            id_caches,
        }
    }

    fn get_last_timestamp(&self) -> i64 {
        return self.id_caches[self.id_cache_index].last_timestamp;
    }

    fn set_last_timestamp(&mut self, time: i64) {
        self.id_caches[self.id_cache_index].last_timestamp = time;
    }

    fn get_sequence(&self) -> u16 {
        self.id_caches[self.id_cache_index].sequence
    }

    fn set_sequence(&mut self, sequence: u16) {
        self.id_caches[self.id_cache_index].sequence = sequence;
    }

    fn gen_id(&mut self, curr_timestamp: i64) -> i64 {
        self.id_caches[self.id_cache_index].gen_id(curr_timestamp, self.standard_epoch)
    }

    pub fn new_multi_thread(worker_id: u16, standard: i64) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(SnowFlakeId::new(worker_id, standard)))
    }

    pub fn generate_id(&mut self) -> Result<i64, String> {
        let mut curr_timestamp = SnowFlakeId::curr_time_millisec();
        let last_timestamp = self.get_last_timestamp();

        if curr_timestamp < last_timestamp {
            return Err(format!(
                "Clock moved backwards.  Refusing to generate id for {} milliseconds in {}",
                last_timestamp, curr_timestamp
            ));
        }

        if curr_timestamp == last_timestamp {
            let pre_sequence = self.get_sequence();
            self.set_sequence((pre_sequence + 1) & SEQUENCE_MASK);
            let sequence = self.get_sequence();
            if sequence == 0 {
                if curr_timestamp == last_timestamp {
                    curr_timestamp = self.wait_for_next_milli_sec();
                }
            }
        } else {
            self.set_sequence(0u16);
        }

        self.set_last_timestamp(curr_timestamp);
        let uid = self.gen_id(curr_timestamp);
        Ok(uid)
    }

    fn wait_for_next_milli_sec(&self) -> i64 {
        let mut curr_timestamp = SnowFlakeId::curr_time_millisec();

        while self.get_last_timestamp() >= curr_timestamp {
            curr_timestamp = SnowFlakeId::curr_time_millisec();
        }

        curr_timestamp
    }

    fn curr_time_millisec() -> i64 {
        let milli_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        milli_sec
    }
}

#[allow(unused_imports)]
mod test {
    use crate::snowflake::{self, SnowFlakeId};
    use std::thread;
    use std::time::Instant;

    #[test]
    fn loop_test() {
        let mut id_gen = SnowFlakeId::new(1, snowflake::STANDARD_EPOCH);
        println!("{:?}", &id_gen);
        let now = Instant::now();
        for _ in 1..1000 {
            let t = &mut id_gen;
            assert!(t.generate_id().is_ok());
        }
        let elapsed = now.elapsed();
        println!(
            "single thread generate 1000 ids cost {}.{:09} s",
            elapsed.as_secs(),
            elapsed.subsec_nanos()
        );
    }

    #[test]
    fn multi_thread() {
        let id_gen = SnowFlakeId::new_multi_thread(2, snowflake::STANDARD_EPOCH);
        let mut ths = Vec::new();
        for i in 1..10 {
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
                println!(
                    "multi thread:[{}] generate 1000 ids cost {}.{:09} s",
                    i,
                    elapsed.as_secs(),
                    elapsed.subsec_nanos()
                );
            }));
        }

        for t in ths {
            let _ = t.join();
        }
    }
}
