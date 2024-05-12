#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::*;

fn func() {
    println!("user_sig_test passed144514551659329349004957724958654498569449286461030793460535628184272357874944802406244404430667271821428459130385895999884310468527595990714181252793313949296235581084957241245373327926509466905647148455127945332028127407363272142441339351427148103208559552479062003558628477632979482672116243850328292663150335627144866959433650713018324921100191246847014505743250440204514425950662545273993064250362182072498757400520308446");
    sigreturn();
}

#[no_mangle]
pub fn main() -> i32 {
    let pid = fork();
    if pid == 0 {
        let mut new = SignalAction::default();
        let mut old = SignalAction::default();
        new.handler = func as usize;

        println!("signal_simple2: child sigaction");
        if sigaction(SIGUSR1, Some(&new), Some(&mut old)) < 0 {
            panic!("Sigaction failed!");
        }
        sleep(1000);
        println!("signal_simple2: child done");
        exit(0);
    } else if pid > 0 {
        println!("signal_simple2: parent kill child");
        sleep(500);
        if kill(pid as usize, SIGUSR1) < 0 {
            println!("Kill failed!");
            exit(1);
        }
        println!("signal_simple2: parent wait child");
        let mut exit_code = 0;
        waitpid(pid as usize, &mut exit_code);
        println!("signal_simple2: parent Done");
        exit(0);
    }

    0
}
