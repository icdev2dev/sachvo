use std::{
    env, 
    io:: {Result,BufReader, prelude::*},
    fs::File,

};

fn main() -> Result<()>{

    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; 

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let lineb = line?;
        
        command_executor::process_line(lineb);

    }

    Ok(())

}

mod command_executor {
    use std::{
        io::Result,
        process::{Command, Stdio},
    
    };

    
    fn execute_command(cmd: String) -> Result<()>  {
        let child = Command::new("/bin/sh")
                            .args(&["-c", &cmd[..]])
                            .stderr(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()?;

        let output = child.wait_with_output()?;
        println!("{:?}", output);
        Ok(())


    }
    pub fn process_line(line:String) {
        let splitline: Vec<&str> = line.split(":").collect();
        if splitline[1].eq_ignore_ascii_case("AUTOMERGE") {
            let cmd = auto_merge(line);
            println!("{}", cmd);
            let _res =  execute_command(cmd).expect("some error");
            
            
        }
    }
    pub fn auto_merge(line: String) -> String {
        let splitline: Vec<&str> = line.split(":").collect();
        let location_dfx = splitline[0];
        

        let mut s = String::from(location_dfx);
        s.push_str(" identity use ");



        s.push_str(splitline[3]);
        s.push_str(" && ");
        s.push_str(splitline[0]);
        s.push_str(" canister --network=ic --no-wallet call rrkah-fqaaa-aaaaa-aaaaq-cai manage_neuron \" \n(record \\\n");
        


        s.push_str("   {\\\n");
        s.push_str("         id=opt (record {id=");

        s.push_str(splitline[4]);
        s.push_str(":nat64}); \\\n");
        s.push_str("         neuron_id_or_subaccount = null; \\\n");
        s.push_str("         command=opt ( variant {MakeProposal=(\\\n");
        s.push_str("                   record { \\\n");
        s.push_str("                       action=opt (variant { ManageNeuron=record { \\\n");
        s.push_str("                             id=opt (record {id =");
        s.push_str(splitline[5]);
        s.push_str(":nat64}); \\\n");

        s.push_str("                             neuron_id_or_subaccount = null; \\\n");
        s.push_str("                             command =  opt( variant { MergeMaturity = record { percentage_to_merge = ");
        s.push_str(splitline[2]);

        s.push_str(" : nat32 } })\\\n");
        s.push_str("                       }});\\\n");
        s.push_str("                       url = \\\"icdev2dev.org\\\";\\\n");
        s.push_str("                       summary = \\\"automerge\\\";\\\n");
        s.push_str("                   }\\\n");
        s.push_str("         )})\\\n");
        s.push_str("   }\\\n");
        
        s.push_str(")\n\"");
        return  s;

    }
}