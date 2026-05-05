use crate::core::process::Process;
use std::vec::Vec;

const CONTEXT_TIME: f32 = 0.2;

fn find_next_process(_queue: &mut Vec<Process>, _timer: f32) -> i32 {
    let mut min_time: f32 = f32::MAX;
    let mut shortest: i32 = -1;

    for i in 0.._queue.len() {
        if _queue[i].tiempo_llegada <= _timer
            && _queue[i].tiempo_restante < min_time
            && _queue[i].tiempo_restante > 0.0
        {
            min_time = _queue[i].tiempo_restante;
            shortest = i as i32;
        }
    }
    shortest
}

pub fn srtf(_queue: &mut Vec<Process>) -> () {
    let mut timer: f32 = 0.0;
    let mut procesos_completados: usize = 0;
    let size_process = _queue.len();

    while procesos_completados < size_process {
        let id = find_next_process(_queue, timer);

        if id != -1 {
            _queue[id as usize].tiempo_restante -= 1.0;
            timer += 1.0;

            if _queue[id as usize].tiempo_restante == 0.0 {
                procesos_completados += 1;
                _queue[id as usize].tiempo_finalizacion = timer;
                _queue[id as usize].turnaround_time =
                    _queue[id as usize].tiempo_finalizacion - _queue[id as usize].tiempo_llegada;
                _queue[id as usize].tiempo_de_espera =
                    _queue[id as usize].turnaround_time - _queue[id as usize].tiempo_rafaga;
            }
        } else {
            timer += 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srtf_algorithm() {
        let mut queue = vec![
            Process::new(1, 0.0, 8.0),
            Process::new(2, 3.0, 4.0),
            Process::new(3, 6.0, 2.0),
            Process::new(4, 10.0, 3.0),
            Process::new(5, 15.0, 6.0),
        ];
        srtf(&mut queue);
        for process in queue {
            println!(
                "P{} | Tiempo de espera: {}",
                process.num, process.tiempo_de_espera
            );
        }
    }
}
