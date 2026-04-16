#[cfg(test)]
mod run_tests {
    use std::fs;
    use serde::{Deserialize, Serialize};
    use serde_json::Value::String;
    use crate::{cpu, memory, ppu};


    #[derive(Serialize, Deserialize)]
    struct TransactionData {
        kind: usize,
        size: usize,
        addr: u32,
        data: u32,
        cycle: usize,
        access: usize,
    }
    #[derive(Serialize, Deserialize)]
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
        let data = fs::read_to_string("C:/GBA Test/test/arm_data_proc_immediate_fixed.json").unwrap();


        let d: Vec<TestData> = serde_json::from_str(data.as_str()).unwrap();
        println!("Loaded Test Data");
        let N = 50;
        for i in 0..N{
            let success = run_test(i, &d[i]);

            assert!(success);
        }


    }

    fn run_test(n:usize, data: &TestData) -> bool {
        println!("\x1b[1;36mRunning Test #{}:\x1b[0;37m", n);

        let mut mem = memory::init();
        let mut cpu = cpu::init();
        let mut ppu = ppu::init();

        cpu.r = data.initial.r.clone();
        let cspr = data.initial.cpsr;
        println!("cspr:{:032b}, {:#4x}", cspr, cspr);
        cpu.n = (cspr & (1 << 31)) != 0;
        cpu.z = (cspr & (1 << 30)) != 0;
        cpu.c = (cspr & (1 << 29)) != 0;
        cpu.v = (cspr & (1 << 28)) != 0;
        cpu.fetch_arm = data.opcode;

        cpu.tick_cycle(&mut mem);
        ppu.tick_cycle(&mut mem);

        let mut same = true;
        for i in 0..16 {
            if cpu.r[i] != data.final_d.r[i] {
                same = false;
            }
        }
        println!("{}",format_regs(data.initial.r));
        if !same {
            let expected = format_regs(data.final_d.r);
            let actual = format_regs(cpu.r);

            let x = data.opcode;
            let str = format!("{:032b}",x);

            print!("Instruction: ");
            for i in 0..8{
                print!("{} ", str.get(i*4..i*4+4).unwrap());
            }
            println!(" ({:#x})", x);


            print!("\x1b[32mExpected: ");
            print!("{expected}");
            print!("\n  \x1b[31mActual: ");
            for i in 0..actual.len(){
                if expected.chars().nth(i) != actual.chars().nth(i){
                    print!("\x1b[31m");
                }else{
                    print!("\x1b[32m");
                }
                print!("{}",actual.chars().nth(i).unwrap());
            }

            println!("\x1b[0;37m");

            return false;
        }
        return true;
    }

    fn format_regs(r:[u32; 16]) -> std::string::String {
        let mut s = "".to_string();
        for i in 0..16{
            s = format!("{}{:02}: {:#4x} ",s,i, r[i]);
        }
        return s;
    }

}