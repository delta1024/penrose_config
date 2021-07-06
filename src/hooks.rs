use penrose::{
    core::{
	hooks::Hook,
	xconnection::XConn,
    },
    Result, WindowManager
};

pub struct AutoStart {
    apps: Vec<String>,
}

impl AutoStart {
    pub fn new(apps: &[&str]) -> Box<Self> {
	let apps = apps.to_vec().iter().map(|x| x.to_string()).collect();
	Box::new(
	    Self{
		apps
	    }
	)
    }
}

impl<X: XConn> Hook<X> for AutoStart {
    fn startup(&mut self, wm: &mut WindowManager<X>) -> Result<()> {
	for i in &self.apps {
	    let message = format!("spawning {}", &i);
	    run_external_drop!(i);
	    wm.log(message)?;
	}
	Ok(())
    }
}
