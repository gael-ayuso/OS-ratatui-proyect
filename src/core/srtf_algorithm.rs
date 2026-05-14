use crate::core::process::Process;
use std::vec::Vec;

// Constantes
const CONTEXT_TIME: f32 = 0.2;

pub struct Srtf {
    pub queue: Vec<Process>,
    pub timer: f32,
    pub procesos_completados: usize,
    pub num_processes: usize,
    last_executed_id_best_process: i32,
    contex_switches: i32,
}

impl Srtf {
    pub fn new(queue: Vec<Process>) -> Self {
        let num_processes = queue.len();
        Self {
            queue,
            timer: 0.0,
            procesos_completados: 0,
            num_processes,
            last_executed_id_best_process: -2,
            contex_switches: 0,
        }
    }

    //Obtiene el proceso con el menor tiempo restante
    fn find_next_process(&self) -> i32 {
        let mut min_time: f32 = f32::MAX;
        let mut shortest: i32 = -1;

        //Recorremos todos los procesos
        for i in 0..self.queue.len() {
            //Si el proceso ha llegado y es menor al tiempo minimo y tiene tiempo restante
            if self.queue[i].tiempo_llegada <= self.timer
                && self.queue[i].tiempo_restante < min_time
                && self.queue[i].tiempo_restante > 0.0
            {
                min_time = self.queue[i].tiempo_restante;
                shortest = i as i32;
            }
        }
        shortest
    }

    pub fn srtf(&mut self) -> f32 {
        while self.procesos_completados < self.num_processes {
            let id_best_process = self.find_next_process();

            let is_normal_preemption = self.last_executed_id_best_process != -1 
                && self.last_executed_id_best_process != -2 
                && id_best_process != -1 
                && self.last_executed_id_best_process != id_best_process;
            let is_after_idle = self.last_executed_id_best_process == -1 && id_best_process != -1;

            //Si hay cambio de proceso, se incrementa el contador de cambios de contexto
            if is_normal_preemption || is_after_idle {
                for i in 0..self.queue.len() {
                    //Si el proceso esta en espera y tiene tiempo restante
                    if self.queue[i].tiempo_restante > 0.0 {
                        if self.queue[i].tiempo_llegada < self.timer {
                            //Incrementa los cambios de contexto del proceso
                            self.queue[i].context_switches += 1;
                        } else if self.queue[i].tiempo_llegada == self.timer {
                            //6. Cuando el tiempo de llegada de un proceso al sistema de cómputo coincida con el instante de un cambio de
                            //contexto, de acuerdo a los objetivos a cumplir por el Sistema Operativo, se dará prioridad al cambio de contexto
                            //correspondiente y se considerará que, el proceso que ha llegado, será recibido posteriormente por el mismo
                            //Sistema Operativo para ser colocado en la cola de procesos listos (memoria RAM) según la consideración 1
                            //--------------------------------------------------------------------------------------------------------
                            //7. Como una extensión de la consideración anterior (la consideración 6), para el proceso cuya llegada coincida
                            // con el instante de un cambio de contexto no se contabilizará el tiempo de cambio de contexto en su tiempo total
                            // de espera, a menos que, según el algoritmo correspondiente, sea el proceso que deba ser colocado en la CPU
                            // para iniciar su procesamiento, en cuyo caso, sí se contabilizará el tiempo de cambio de contexto en su tiempo
                            // total de espera ya que el algoritmo de planificación lo deberá llevar de la cola de procesos listos (memoria RAM)
                            // a la CPU.
                            if i == id_best_process as usize {
                                self.queue[i].context_switches += 1;
                            }
                        }
                    }
                }
                //Incrementa los cambios de contexto globales
                self.contex_switches += 1;
            }
            self.last_executed_id_best_process = id_best_process;

            let decrement = if self.queue[id_best_process as usize].tiempo_restante < 1.0 {
                self.queue[id_best_process as usize].tiempo_restante
            } else {
                1.0
            };

            self.queue[id_best_process as usize].tiempo_restante -= decrement;
            self.timer += decrement;
            self.queue[id_best_process as usize].time_last_execution += decrement;

            //Si el proceso termina
            if self.queue[id_best_process as usize].tiempo_restante <= 0.0 {
                self.queue[id_best_process as usize].tiempo_restante = 0.0;
                self.procesos_completados += 1;
                //Tiempo de finalizacion = tiempo actual + tiempo de cambios de contexto
                self.queue[id_best_process as usize].tiempo_finalizacion = self.timer
                    + (self.queue[id_best_process as usize].context_switches as f32 * CONTEXT_TIME);
                //Tiempo en la CPU = tiempo de finalizacion - tiempo de llegada
                self.queue[id_best_process as usize].turnaround_time =
                    self.queue[id_best_process as usize].tiempo_finalizacion
                        - self.queue[id_best_process as usize].tiempo_llegada;
                //Tiempo de espera = tiempo en la CPU - tiempo de rafaga
                self.queue[id_best_process as usize].tiempo_de_espera =
                    self.queue[id_best_process as usize].turnaround_time
                        - self.queue[id_best_process as usize].tiempo_rafaga;
            } else {
                self.timer += 1.0;
            }
        }

        self.timer
    }

    pub fn execute_one_process(&mut self) -> Process {
        if self.procesos_completados >= self.num_processes {
            return Process::new(0, 0.0, 0.0);
        }

        let id_best_process = self.find_next_process();

        //Se define el proceso "muerto" si no hay procesos pendientes
        let mut wasted_process = if id_best_process == -1 {
            Process::new(0, self.timer, 0.0)
        } else {
            Process::new(0, 0.0, 0.0)
        };

        let is_normal_preemption = self.last_executed_id_best_process != -1 
            && self.last_executed_id_best_process != -2 
            && id_best_process != -1 
            && self.last_executed_id_best_process != id_best_process;
        let is_after_idle = self.last_executed_id_best_process == -1 && id_best_process != -1;

        //Si hay cambio de proceso, se incrementa el contador de cambios de contexto
        if is_normal_preemption || is_after_idle {
            for i in 0..self.queue.len() {
                //Si el proceso esta en espera y tiene tiempo restante
                if self.queue[i].tiempo_restante > 0.0 {
                    if self.queue[i].tiempo_llegada < self.timer {
                        //Incrementa los cambios de contexto del proceso
                        self.queue[i].context_switches += 1;
                    } else if self.queue[i].tiempo_llegada == self.timer {
                        //6. Cuando el tiempo de llegada de un proceso al sistema de cómputo coincida con el instante de un cambio de
                        //contexto, de acuerdo a los objetivos a cumplir por el Sistema Operativo, se dará prioridad al cambio de contexto
                        //correspondiente y se considerará que, el proceso que ha llegado, será recibido posteriormente por el mismo
                        //Sistema Operativo para ser colocado en la cola de procesos listos (memoria RAM) según la consideración 1
                        //--------------------------------------------------------------------------------------------------------
                        //7. Como una extensión de la consideración anterior (la consideración 6), para el proceso cuya llegada coincida
                        // con el instante de un cambio de contexto no se contabilizará el tiempo de cambio de contexto en su tiempo total
                        // de espera, a menos que, según el algoritmo correspondiente, sea el proceso que deba ser colocado en la CPU
                        // para iniciar su procesamiento, en cuyo caso, sí se contabilizará el tiempo de cambio de contexto en su tiempo
                        // total de espera ya que el algoritmo de planificación lo deberá llevar de la cola de procesos listos (memoria RAM)
                        // a la CPU.
                        if i == id_best_process as usize {
                            self.queue[i].context_switches += 1;
                        }
                    }
                }
            }
            //Incrementa los cambios de contexto globales
            self.contex_switches += 1;
        }

        self.last_executed_id_best_process = id_best_process;

        if id_best_process != -1 {
            self.queue[id_best_process as usize].time_last_execution = 0.0;
        }

        loop {
            let id_best_process2 = self.find_next_process();

            //Si el proceso cambia o no hay procesos pendientes
            if id_best_process2 != id_best_process {
                break;
            }

            if id_best_process != -1 {
                let decrement = if self.queue[id_best_process as usize].tiempo_restante < 1.0 {
                    self.queue[id_best_process as usize].tiempo_restante
                } else {
                    1.0
                };

                self.queue[id_best_process as usize].tiempo_restante -= decrement;
                self.timer += decrement;
                self.queue[id_best_process as usize].time_last_execution += decrement;

                //Si el proceso termina
                if self.queue[id_best_process as usize].tiempo_restante <= 0.0 {
                    self.queue[id_best_process as usize].tiempo_restante = 0.0;
                    self.procesos_completados += 1;
                    //Tiempo de finalizacion = tiempo actual + tiempo de cambios de contexto
                    self.queue[id_best_process as usize].tiempo_finalizacion = self.timer
                        + (self.queue[id_best_process as usize].context_switches as f32
                            * CONTEXT_TIME);
                    //Tiempo en la CPU = tiempo de finalizacion - tiempo de llegada
                    self.queue[id_best_process as usize].turnaround_time =
                        self.queue[id_best_process as usize].tiempo_finalizacion
                            - self.queue[id_best_process as usize].tiempo_llegada;
                    //Tiempo de espera = tiempo en la CPU - tiempo de rafaga
                    self.queue[id_best_process as usize].tiempo_de_espera =
                        self.queue[id_best_process as usize].turnaround_time
                            - self.queue[id_best_process as usize].tiempo_rafaga;
                }
            } else {
                //Tiempo muerto de la CPU
                wasted_process.tiempo_rafaga += 1.0;
                self.timer += 1.0;
            }

            if self.procesos_completados >= self.num_processes {
                break;
            }
        }

        if id_best_process != -1 {
            self.queue[id_best_process as usize]
        } else {
            wasted_process
        }
    }

    //Calcula el tiempo promedio de espera de todos los procesos
    pub fn calculate_average_waiting_time(&self) -> f32 {
        let mut total_waiting_time = 0.0;
        for process in &self.queue {
            total_waiting_time += process.tiempo_de_espera;
        }
        total_waiting_time / self.queue.len() as f32
    }

    //Obtiene el tiempo total que se tarda en ejecutar todos los procesos
    pub fn get_total_process_time(&self) -> f32 {
        self.timer + (self.contex_switches as f32 * CONTEXT_TIME)
    }

    //Calcula el porcentaje de tiempo de espera respecto al tiempo total
    pub fn tte_ttp(&self) -> f32 {
        (self.calculate_average_waiting_time() / self.get_total_process_time()) * 100.0
    }

    pub fn reset_srtf(&mut self, processes: Vec<Process>) {
        self.queue = processes;
        self.timer = 0.0;
        self.procesos_completados = 0;
        self.num_processes = self.queue.len() as usize;
        self.last_executed_id_best_process = -2;
        self.contex_switches = 0;
    }

    pub fn has_next_process(&self) -> bool {
        self.procesos_completados < self.num_processes
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_srtf_algorithm() {
        let queue = vec![
            Process::new(1, 0.0, 8.0),
            Process::new(2, 3.0, 4.0),
            Process::new(3, 6.0, 2.0),
            Process::new(4, 10.0, 3.0),
            Process::new(5, 15.0, 6.0),
        ];
        let mut srtf = Srtf::new(queue);
        let timer = srtf.srtf();
        for process in &srtf.queue {
            println!(
                "P{} | Tiempo de espera: {}",
                process.num, process.tiempo_de_espera
            );
        }
        println!("Tiempo total: {}", timer);
    }

    #[test]
    fn test_execute_one_process() {
        let queue = vec![
            Process::new(1, 0.0, 8.0),
            Process::new(2, 3.0, 4.0),
            Process::new(3, 6.0, 2.0),
            Process::new(4, 10.0, 3.0),
            Process::new(5, 20.0, 1.0),
        ];
        let mut srtf = Srtf::new(queue);
        let process = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process);
        let process_2 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_2);
        let process_3 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_3);
        let process_4 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_4);
        let process_5 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_5);
        let process_6 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_6);
        let process_7 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_7);
        let process_8 = srtf.execute_one_process();
        println!("Proceso ejecutado: {:?}", process_8);
        assert!(process_8.context_switches == 1);
        println!("Cambios de contexto: {}", srtf.contex_switches);

        println!("Tiempo total: {}", srtf.get_total_process_time());
        assert_eq!(srtf.contex_switches, 6);
    }

    #[test]
    fn test_wasted_time() {
        let queue = vec![
            Process::new(1, 0.0, 1.0),
            Process::new(2, 5.0, 1.0),
            Process::new(3, 7.0, 2.0),
        ];

        let mut srtf = Srtf::new(queue);
        while srtf.has_next_process() {
            let process = srtf.execute_one_process();
            println!("Proceso ejecutado: {:?}", process);
        }
    }
}
