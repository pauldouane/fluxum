use crate::error::ConfigError;
use crate::job::{Job, Operator, Status};
use crate::logger::Logger;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::{Mutex, MutexGuard};

fn clear_values(
    name: &mut Vec<u8>,
    id: &mut Vec<u8>,
    run: &mut Vec<u8>,
    type_preset: &mut Vec<u8>,
) {
    name.clear();
    id.clear();
    run.clear();
    type_preset.clear();
}

pub struct Config {
    pub jobs: Vec<Job>,
}

impl Config {
    pub async fn add(
        &mut self,
        name: Vec<u8>,
        id: Vec<u8>,
        run: Vec<u8>,
        type_preset: Vec<u8>,
    ) -> bool {
        let type_preset = match std::str::from_utf8(&type_preset) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 type sequence: {}", e),
        };
        let mut operator: Operator = match type_preset {
            "shell" => Operator::ShellOperator,
            "python" => Operator::PythonOperator,
            &_ => panic!("Invalid type sequence"),
        };
        self.jobs.push(Job {
            name,
            id,
            run,
            status: Status::NoStatus,
            operator,
        });
        true
    }

    // Process based on ASCII table : search value by octet
    pub async fn get_jobs_by_config(
        &mut self,
        logger: Arc<Mutex<Logger>>,
    ) -> Result<(), ConfigError> {
        let mut logger: MutexGuard<Logger> = logger.lock().await;
        let mut f = match File::open("workflow.yaml").await {
            Ok(f) => {
                logger.log("Config File has been read", "info");
                f
            }
            Err(err) => return Err(ConfigError::FileNotFoundError(err)),
        };
        let mut buffer: Vec<u8> = Vec::new();
        match f.read_to_end(&mut buffer).await {
            Ok(_) => {
                logger.log("Buffer has been set", "info");
            }
            Err(err) => println!("{}", err),
        }
        // Index of property
        let mut index: usize = 0;
        // Store bytes in the appropriate property
        let (mut name, mut id, mut run, mut type_preset): (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) =
            (vec![], vec![], vec![], vec![]);
        // Control iterator loop
        let mut i = 0;
        while i < buffer.len() {
            // New job
            if buffer[i] == 0x2D && buffer[i + 1] == 0x20 {
                if i != 0 {
                    self.add(name.clone(), id.clone(), run.clone(), type_preset.clone())
                        .await;
                    clear_values(&mut name, &mut id, &mut run, &mut type_preset);
                }
                i += 2;
                index = 0;
                continue;
            }
            // New property for actual job
            if buffer[i] == 0x20 && buffer[i + 1] == 0x20 {
                i += 2;
                index += 1;
                continue;
            }
            // Name property
            if buffer[i] == 0x6E
                && buffer[i + 1] == 0x61
                && buffer[i + 2] == 0x6D
                && buffer[i + 3] == 0x65
                && buffer[i + 4] == 0x3A
            {
                i += 6;
                continue;
            }
            // Type  property
            if (i + 3) < buffer.len() {
                if buffer[i] == 0x74
                    && buffer[i + 1] == 0x79
                    && buffer[i + 2] == 0x70
                    && buffer[i + 3] == 0x65
                {
                    i += 6;
                    continue;
                }
            }
            // Id property
            if buffer[i] == 0x69 && buffer[i + 1] == 0x64 {
                i += 4;
                continue;
            }
            // Run property
            if buffer[i] == 0x72 && buffer[i + 1] == 0x75 && buffer[i + 2] == 0x6E {
                i += 5;
                continue;
            }
            // If bytes is not \n
            if buffer[i] != 0xA && buffer[i] != 13 {
                match index {
                    0 => name.push(buffer[i]),
                    1 => id.push(buffer[i]),
                    2 => type_preset.push(buffer[i]),
                    3 => run.push(buffer[i]),
                    _ => {}
                }
            }
            // Push last job
            if i == (buffer.len() - 1) {
                self.add(name.clone(), id.clone(), run.clone(), type_preset.clone())
                    .await;
            }
            i += 1;
        }
        Ok(())
    }
}
