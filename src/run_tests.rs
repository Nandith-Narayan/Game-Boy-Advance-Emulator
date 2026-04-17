#[cfg(test)]
mod run_tests {
    use std::fs;
    use serde::{Deserialize, Serialize};
    use crate::{cpu, memory, ppu};
    use crate::cpu::Cpu;
    use crate::cpu::enums::CPUMode::*;
    use crate::cpu::enums::RegisterName::*;

    #[derive(Serialize, Deserialize)]
    struct TransactionData {
        kind: usize,
        size: usize,
        addr: u32,
        data: u32,
        cycle: usize,
        access: usize,
    }
    #[derive(Serialize, Deserialize, Copy, Clone)]
    struct State {
        #[serde(rename = "R")]
        r: [u32; 16],
        #[serde(rename = "R_fiq")]
        r_fiq: [u32; 7],
        #[serde(rename = "R_svc")]
        r_svc: [u32; 2],
        #[serde(rename = "R_abt")]
        r_abt: [u32; 2],
        #[serde(rename = "R_irq")]
        r_irq: [u32; 2],
        #[serde(rename = "R_und")]
        r_und: [u32; 2],
        #[serde(rename = "CPSR")]
        cpsr: u32,
        #[serde(rename = "SPSR")]
        spsr: [u32; 5],
        pipeline: [u32; 2],
    }

    #[derive(Serialize, Deserialize)]
    struct TestData {
        initial: State,
        #[serde(rename = "final")]
        final_d: State,
        transactions: Vec<TransactionData>,
        opcode: u32,
        base_addr: u32,

    }


    #[test]
    fn test_json() {
        let tests_to_skip = [55];

        let data = fs::read_to_string("C:/GBA Test/test/arm_data_proc_immediate_fixed.json").unwrap();


        let d: Vec<TestData> = serde_json::from_str(data.as_str()).unwrap();
        println!("Loaded Test Data");
        let N = 100;
        for i in 0..N{
            if tests_to_skip.contains(&i){
                println!("\x1b[1;31mSkipping Test #{}!\x1b[0;37m", i);
                continue;
            }
            let success = run_test(i, &d[i]);

            assert!(success);
        }


    }

    fn run_test(n:usize, data: &TestData) -> bool {
        println!("\x1b[1;36mRunning Test #{}:\x1b[0;37m", n);

        let mut mem = memory::init();
        let mut cpu = cpu::init();
        let mut ppu = ppu::init();

        load_cpu(&mut cpu, data.initial);
        let cpsr = data.initial.cpsr;
        //println!("cpsr:{:032b}, {:#4x}", cpsr, cpsr);
        //println!("new cpsr:{:032b}, {:#4x}", data.final_d.cpsr, data.final_d.cpsr);
        cpu.n = (cpsr & (1 << 31)) != 0;
        cpu.z = (cpsr & (1 << 30)) != 0;
        cpu.c = (cpsr & (1 << 29)) != 0;
        cpu.v = (cpsr & (1 << 28)) != 0;

        cpu.fetch_arm = data.opcode;
        cpu.decode_arm(&mut mem);

        cpu.tick_cycle(&mut mem);
        ppu.tick_cycle(&mut mem);



        let mut same = compare_regs(&mut cpu, data.final_d);

        if !same {
            let expected = format_regs(data.final_d.r);

            let x = data.opcode;
            let str = format!("{:032b}",x);


            print!("Instruction: ");
            for i in 0..8{
                print!("{} ", str.get(i*4..i*4+4).unwrap());
            }
            println!(" ({:#x})", x);

            display_regs(&mut cpu, data.final_d, data.initial);

            return false;
        }
        return true;
    }

    fn display_reg(name: &str, actual: u32, expected: u32, initial: u32){
        let bold = "\x1b[1m";
        let red = "\x1b[31m";
        let green = "\x1b[32m";
        let clear = "\x1b[0;37m";
        print!("{bold}{green}{:<6}| ", name);
        if actual != expected {
            print!("{red}");
        }
        print!("{:>#10x} {green}", actual);
        println!("| {:>#10x} | {:>#10x} |{clear}", expected, initial);
    }
    fn display_bool(name: &str, actual: bool, expected: bool, initial: bool){
        let bold = "\x1b[1m";
        let red = "\x1b[31m";
        let green = "\x1b[32m";
        let clear = "\x1b[0;37m";
        print!("{bold}{green}{:<6}| ", name);
        if actual != expected {
            print!("{red}");
        }
        print!("{:^10} {green}", actual);
        println!("| {:^10} | {:^10} |{clear}", expected, initial);
    }
    fn display_regs(cpu: &mut Cpu, expected: State, initial: State){
        println!("\x1b[32mReg   |{:^12}|{:^12}|{:^12}|","Actual", "Expected", "Initial");
        display_reg(&*format!("{:?}", R0).to_string(), cpu.regs[R0 as usize], expected.r[0], initial.r[0]);
        display_reg(&*format!("{:?}", R1).to_string(), cpu.regs[R1 as usize], expected.r[1], initial.r[1]);
        display_reg(&*format!("{:?}", R2).to_string(), cpu.regs[R2 as usize], expected.r[2], initial.r[2]);
        display_reg(&*format!("{:?}", R3).to_string(), cpu.regs[R3 as usize], expected.r[3], initial.r[3]);
        display_reg(&*format!("{:?}", R4).to_string(), cpu.regs[R4 as usize], expected.r[4], initial.r[4]);
        display_reg(&*format!("{:?}", R5).to_string(), cpu.regs[R5 as usize], expected.r[5], initial.r[5]);
        display_reg(&*format!("{:?}", R6).to_string(), cpu.regs[R6 as usize], expected.r[6], initial.r[6]);
        display_reg(&*format!("{:?}", R7).to_string(), cpu.regs[R7 as usize], expected.r[7], initial.r[7]);
        display_reg(&*format!("{:?}", R8).to_string(), cpu.regs[R8 as usize], expected.r[8], initial.r[8]);
        display_reg(&*format!("{:?}", R9).to_string(), cpu.regs[R9 as usize], expected.r[9], initial.r[9]);
        display_reg(&*format!("{:?}", R10).to_string(), cpu.regs[R10 as usize], expected.r[10], initial.r[10]);
        display_reg(&*format!("{:?}", R11).to_string(), cpu.regs[R11 as usize], expected.r[11], initial.r[11]);
        display_reg(&*format!("{:?}", R12).to_string(), cpu.regs[R12 as usize], expected.r[12], initial.r[12]);
        display_reg(&*format!("{:?}", R13).to_string(), cpu.regs[R13 as usize], expected.r[13], initial.r[13]);
        display_reg(&*format!("{:?}", R14).to_string(), cpu.regs[R14 as usize], expected.r[14], initial.r[14]);
        display_reg(&*format!("{:?}", R15).to_string(), cpu.regs[R15 as usize], expected.r[15], initial.r[15]);
        display_reg(&*format!("{:?}", R8FIQ).to_string(), cpu.regs[R8FIQ as usize], expected.r_fiq[0], initial.r_fiq[0]);
        display_reg(&*format!("{:?}", R9FIQ).to_string(), cpu.regs[R9FIQ as usize], expected.r_fiq[1], initial.r_fiq[1]);
        display_reg(&*format!("{:?}", R10FIQ).to_string(), cpu.regs[R10FIQ as usize], expected.r_fiq[2], initial.r_fiq[2]);
        display_reg(&*format!("{:?}", R11FIQ).to_string(), cpu.regs[R11FIQ as usize], expected.r_fiq[3], initial.r_fiq[3]);
        display_reg(&*format!("{:?}", R12FIQ).to_string(), cpu.regs[R12FIQ as usize], expected.r_fiq[4], initial.r_fiq[4]);
        display_reg(&*format!("{:?}", R13FIQ).to_string(), cpu.regs[R13FIQ as usize], expected.r_fiq[5], initial.r_fiq[5]);
        display_reg(&*format!("{:?}", R14FIQ).to_string(), cpu.regs[R14FIQ as usize], expected.r_fiq[6], initial.r_fiq[6]);
        display_reg(&*format!("{:?}", R13IRQ).to_string(), cpu.regs[R13IRQ as usize], expected.r_irq[0], initial.r_irq[0]);
        display_reg(&*format!("{:?}", R14IRQ).to_string(), cpu.regs[R14IRQ as usize], expected.r_irq[1], initial.r_irq[1]);
        display_reg(&*format!("{:?}", R13SVC).to_string(), cpu.regs[R13SVC as usize], expected.r_svc[0], initial.r_svc[0]);
        display_reg(&*format!("{:?}", R14SVC).to_string(), cpu.regs[R14SVC as usize], expected.r_svc[1], initial.r_svc[1]);
        display_reg(&*format!("{:?}", R13ABT).to_string(), cpu.regs[R13ABT as usize], expected.r_abt[0], initial.r_abt[0]);
        display_reg(&*format!("{:?}", R14ABT).to_string(), cpu.regs[R14ABT as usize], expected.r_abt[1], initial.r_abt[1]);
        display_reg(&*format!("{:?}", R13UND).to_string(), cpu.regs[R13UND as usize], expected.r_und[0], initial.r_und[0]);
        display_reg(&*format!("{:?}", R14UND).to_string(), cpu.regs[R14UND as usize], expected.r_und[1], initial.r_und[1]);
        display_reg("CPSR", cpu.get_cpsr(), expected.cpsr, initial.cpsr);

        display_bool("N", cpu.n, (expected.cpsr & (1 << 31)) != 0, (initial.cpsr & (1 << 31)) != 0);
        display_bool("Z", cpu.z, (expected.cpsr & (1 << 30)) != 0, (initial.cpsr & (1 << 30)) != 0);
        display_bool("C", cpu.c, (expected.cpsr & (1 << 29)) != 0, (initial.cpsr & (1 << 29)) != 0);
        display_bool("V", cpu.v, (expected.cpsr & (1 << 28)) != 0, (initial.cpsr & (1 << 28)) != 0);
    }
    fn load_cpu(cpu: &mut Cpu, data: State){
        let mode = cpu.num_to_cpu_mode(data.cpsr & 0b1_1111);
        match mode{
            USER | SYSTEM => {
                cpu.reg_map[8] = R8 as usize;
                cpu.reg_map[9] = R9 as usize;
                cpu.reg_map[10] = R10 as usize;
                cpu.reg_map[11] = R11 as usize;
                cpu.reg_map[12] = R12 as usize;
                cpu.reg_map[13] = R13 as usize;
                cpu.reg_map[14] = R14 as usize;
            }
            FIQ => {
                cpu.reg_map[8] = R8FIQ as usize;
                cpu.reg_map[9] = R9FIQ as usize;
                cpu.reg_map[10] = R10FIQ as usize;
                cpu.reg_map[11] = R11FIQ as usize;
                cpu.reg_map[12] = R12FIQ as usize;
                cpu.reg_map[13] = R13FIQ as usize;
                cpu.reg_map[14] = R14FIQ as usize;
            },
            IRQ => {
                cpu.reg_map[8] = R8 as usize;
                cpu.reg_map[9] = R9 as usize;
                cpu.reg_map[10] = R10 as usize;
                cpu.reg_map[11] = R11 as usize;
                cpu.reg_map[12] = R12 as usize;
                cpu.reg_map[13] = R13IRQ as usize;
                cpu.reg_map[14] = R14IRQ as usize;

            },
            SUPERVISOR => {
                cpu.reg_map[8] = R8 as usize;
                cpu.reg_map[9] = R9 as usize;
                cpu.reg_map[10] = R10 as usize;
                cpu.reg_map[11] = R11 as usize;
                cpu.reg_map[12] = R12 as usize;
                cpu.reg_map[13] = R13SVC as usize;
                cpu.reg_map[14] = R14SVC as usize;

            },
            ABORT => {
                cpu.reg_map[8] = R8 as usize;
                cpu.reg_map[9] = R9 as usize;
                cpu.reg_map[10] = R10 as usize;
                cpu.reg_map[11] = R11 as usize;
                cpu.reg_map[12] = R12 as usize;
                cpu.reg_map[13] = R13ABT as usize;
                cpu.reg_map[14] = R14ABT as usize;

            },
            UNDEFINED => {
                cpu.reg_map[8] = R8 as usize;
                cpu.reg_map[9] = R9 as usize;
                cpu.reg_map[10] = R10 as usize;
                cpu.reg_map[11] = R11 as usize;
                cpu.reg_map[12] = R12 as usize;
                cpu.reg_map[13] = R13UND as usize;
                cpu.reg_map[14] = R14UND as usize;

            },
        }
        cpu.mode = mode;
        
        cpu.regs[R0 as usize] = data.r[0];
        cpu.regs[R1 as usize] = data.r[1];
        cpu.regs[R2 as usize] = data.r[2];
        cpu.regs[R3 as usize] = data.r[3];
        cpu.regs[R4 as usize] = data.r[4];
        cpu.regs[R5 as usize] = data.r[5];
        cpu.regs[R6 as usize] = data.r[6];
        cpu.regs[R7 as usize] = data.r[7];
        cpu.regs[R8 as usize] = data.r[8];
        cpu.regs[R9 as usize] = data.r[9];
        cpu.regs[R10 as usize] = data.r[10];
        cpu.regs[R11 as usize] = data.r[11];
        cpu.regs[R12 as usize] = data.r[12];
        cpu.regs[R13 as usize] = data.r[13];
        cpu.regs[R14 as usize] = data.r[14];
        cpu.regs[R15 as usize] = data.r[15];
        cpu.regs[R8FIQ as usize] = data.r_fiq[0];
        cpu.regs[R9FIQ as usize] = data.r_fiq[1];
        cpu.regs[R10FIQ as usize] = data.r_fiq[2];
        cpu.regs[R11FIQ as usize] = data.r_fiq[3];
        cpu.regs[R12FIQ as usize] = data.r_fiq[4];
        cpu.regs[R13FIQ as usize] = data.r_fiq[5];
        cpu.regs[R14FIQ as usize] = data.r_fiq[6];
        cpu.regs[R13IRQ as usize] = data.r_irq[0];
        cpu.regs[R14IRQ as usize] = data.r_irq[1];
        cpu.regs[R13SVC as usize] = data.r_svc[0];
        cpu.regs[R14SVC as usize] = data.r_svc[1];
        cpu.regs[R13ABT as usize] = data.r_abt[0];
        cpu.regs[R14ABT as usize] = data.r_abt[1];
        cpu.regs[R13UND as usize] = data.r_und[0];
        cpu.regs[R14UND as usize] = data.r_und[1];
    }
    
    fn compare_regs(cpu: &mut Cpu, data: State) -> bool{
        
        let result = 
        cpu.regs[R0 as usize] != data.r[0]||
        cpu.regs[R1 as usize] != data.r[1]||
        cpu.regs[R2 as usize] != data.r[2]||
        cpu.regs[R3 as usize] != data.r[3]||
        cpu.regs[R4 as usize] != data.r[4]||
        cpu.regs[R5 as usize] != data.r[5]||
        cpu.regs[R6 as usize] != data.r[6]||
        cpu.regs[R7 as usize] != data.r[7]||
        cpu.regs[R8 as usize] != data.r[8]||
        cpu.regs[R9 as usize] != data.r[9]||
        cpu.regs[R10 as usize] != data.r[10]||
        cpu.regs[R11 as usize] != data.r[11]||
        cpu.regs[R12 as usize] != data.r[12]||
        cpu.regs[R13 as usize] != data.r[13]||
        cpu.regs[R14 as usize] != data.r[14]||
        cpu.regs[R15 as usize] != data.r[15]||
        cpu.regs[R8FIQ as usize] != data.r_fiq[0]||
        cpu.regs[R9FIQ as usize] != data.r_fiq[1]||
        cpu.regs[R10FIQ as usize] != data.r_fiq[2]||
        cpu.regs[R11FIQ as usize] != data.r_fiq[3]||
        cpu.regs[R12FIQ as usize] != data.r_fiq[4]||
        cpu.regs[R13FIQ as usize] != data.r_fiq[5]||
        cpu.regs[R14FIQ as usize] != data.r_fiq[6]||
        cpu.regs[R13IRQ as usize] != data.r_irq[0]||
        cpu.regs[R14IRQ as usize] != data.r_irq[1]||
        cpu.regs[R13SVC as usize] != data.r_svc[0]||
        cpu.regs[R14SVC as usize] != data.r_svc[1]||
        cpu.regs[R13ABT as usize] != data.r_abt[0]||
        cpu.regs[R14ABT as usize] != data.r_abt[1]||
        cpu.regs[R13UND as usize] != data.r_und[0]||
        cpu.regs[R14UND as usize] != data.r_und[1]||
        (cpu.get_cpsr()&0xF0000000) != (data.cpsr&0xF0000000);

        return !result;
    }
    
    
    fn format_regs(r:[u32; 16]) -> std::string::String {
        let mut s = "".to_string();
        for i in 0..16{
            s = format!("{}{:02}: {:#4x} ",s,i, r[i]);
        }
        return s;
    }

}