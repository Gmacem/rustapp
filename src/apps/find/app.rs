use std::path::{PathBuf};

use crate::apps::find::args::FindAppArgs;
use crate::apps::find::strategies::{
    Context, FindProcess, PostProcessStrategy, PrintConsoleStrategy, PrintFileStrategy,
    PrintStrategy, ProcessStrategy,
};

use super::strategies::{SortStrategy, MultiplePostProcess, InTextFileFilter};

pub struct FindApp {
    request: String,
    process_strategy: Box<dyn ProcessStrategy>,
    post_process_strategy: Option<Box<dyn PostProcessStrategy>>,
    print_strategy: Option<Box<dyn PrintStrategy>>,
}

impl FindApp {
    pub fn new(root: PathBuf, args: FindAppArgs) -> FindApp {
        let mut find_app = FindApp {
            request: args.name,
            process_strategy: Box::new(FindProcess::new(root)),
            post_process_strategy: None,
            print_strategy: None,
        };
        let mut post_strategies = Box::new(MultiplePostProcess::new());
        if args.sort {
            post_strategies.add_strategy(Box::new(SortStrategy {}));
        }
        if let Some(content) = args.in_file {
            let content = content;
            post_strategies.add_strategy(Box::new(InTextFileFilter::new(content)));
        }

        find_app.post_process_strategy = Some(post_strategies);
        
        if args.filename.is_some() {
            let filename = args.filename.as_ref().unwrap();
            let path = PathBuf::from(filename);
            find_app.print_strategy = Some(Box::new(PrintFileStrategy::new(path)));
        } else {
            find_app.print_strategy = Some(Box::new(PrintConsoleStrategy {}));
        }
        find_app
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut context = Context {
            name: self.request.clone(),
            files: Vec::new(),
        };
        match self.process_strategy.process(&mut context) {
            Ok(()) => (),
            Err(err) => return Err(err),
        };
        if let Some(ref mut strategy) = self.post_process_strategy {
            match strategy
                .post_process(&mut context)
            {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        }
        if let Some(ref mut print_strategy) = self.print_strategy {
            match print_strategy.print(&mut context) {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}
