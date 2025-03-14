extern crate colored;
extern crate whoami;
extern crate nix;
extern crate humantime;
extern crate structopt;

use colored::*;
use whoami::*;
use nix::*;
use humantime::format_duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A rust-y clone of pfetch.",author = "Raul Perdomo <raul@perdomo.org>", no_version, )]
struct Options {
  #[structopt(short, long)]
  ///Make Ferris cuter. 
  cute: bool,

  #[structopt(short, long)]
  ///Mini Ferris mode.
  mini: bool,

}


fn main() {
    let little_ferris =
    "    
    _~^~~^~_
\\) /  o  o  \\  (/
  '_   __   _'  
  | '------' |  
  |          | ";

    let cute_ferris = "
              ..  :!~..!7^ :!~  ..                  
         ..  ~?!~7??77?7?7???~~7?. ..               
          ~?7!77???77777777777??7?7!77.             
      .!~~!???77777777777777777777??77~~!^          
    ...7???777777777777777777777777777???^ .        
    !777777777777777777777777777777777777777^       
  ..^?77777777777777??777777777?7777777777?7:..     
 .!77777777777777JPJ:~7777775P~^77777777777777^     
 .~777777777777?!Y@G  :7777^B@! :77777777777?7^.    
^7?777??7777777?!.:   :7777: .  .77777777??777?7:   
7?777!!777777????7~^^~777777~^^!7???777?7!!!777?^   
^7?777^~~7?7!!!77777777???77777777!!!7?!^~~777?!    
 .!???~.^~~77!^~77777!~::::~!7777!~!77~.~^^???~     
   :!??: . .^!???77???7!  !7???77??7^. .. ~?7^      
     :!!     .7?777~~!!!:^7!!!!77??~      7!.       
       .      .~7??!.        :!?7!^       .         
                .::^:        :^:.                   
";             

   let happy_ferris = "                                         
                  :  7~.~?:.7^ ..                 
              .^.^J?7?????????7??.:^              
    .       :.^J??????????????????J?.:.     :~  . 
 !: !?~     7J???????????????????????J^   .?J! .?.
.?J^^?J:  7?????????????????????????????~ :J?7!J~ 
 .~7??! :^7??????????J77????J?7?????????!^.^J?~.  
    .7~.^???????????7&Y :??!P# :?????????7~!^     
      :?????????????~.  :JJ!.  :???????????~      
      !J7:~~~77??????7!7?~^~7!7?????7!~:^:!J!     
       ^7~ .   ..::^^~~!!^:^!~~^::..   ...?^      
         ^~.                             !.       
           :                            :    ";     
   
    let options = Options::from_args();
    let mut crab = String::new();

    if options.cute {
    ///Figure out which version to print.
      crab = cute_ferris.to_string();
    } else if options.mini{
      crab = little_ferris.to_string();
    } else {
      crab = happy_ferris.to_string();
    }

    let mut crab_width = if options.mini { 20 } else { 53 };
    let systeminfo = sys::sysinfo::sysinfo().unwrap();

    let lines = crab.lines();
    let lines_count = &lines.clone().count();
    for (idx, line) in lines.enumerate() {
      match idx {
        1 =>       println!("{}", format!("{:<crab_width$} {}@{}", line.bright_red(), whoami::username(), whoami::hostname())),
        2 =>       println!("{}", format!("{:<crab_width$} {:<6} {}", line.bright_red(), "OS".bold().red().dimmed(), whoami::distro())),
        3 =>       println!("{}", format!("{:<crab_width$} {:<6} {}", line.bright_red(), "Host".bold().red().dimmed(), whoami::hostname())),
        4 =>       println!("{}", format!("{:<crab_width$} {:<6} {} {}", line.bright_red(), "Kernel".bold().red().dimmed(), sys::utsname::uname().unwrap().sysname().to_str().unwrap(), sys::utsname::uname().unwrap().release().to_str().unwrap())),
        5 =>       println!("{}", format!("{:<crab_width$} {:<6} {}", line.bright_red(), "Arch".bold().red().dimmed(), whoami::arch())),
        6 =>       println!("{}", format!("{:<crab_width$} {:<6} {}M / {}M", line.bright_red(), "Memory".bold().red().dimmed(), (systeminfo.ram_total() - &systeminfo.ram_unused())/1000000, &systeminfo.ram_total()/1000000)),
        7 =>       println!("{}", format!("{:<crab_width$} {:<6} {:.2}/{:.2}/{:.2}", line.bright_red(), "Load".bold().red().dimmed(), systeminfo.load_average().0, systeminfo.load_average().1, systeminfo.load_average().2)),
        8 =>       println!("{}", format!("{:<crab_width$} {:<6} {}", line.bright_red(), "Uptime".bold().red().dimmed(), format_duration(systeminfo.uptime()))),
        _ =>       println!("{}", format!("{:<65}", line.bright_red())),

      }

    }
}