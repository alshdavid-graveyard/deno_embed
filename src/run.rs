use std::path::PathBuf;
use std::sync::Arc;
use std::thread::JoinHandle;

use tokio::sync::oneshot;

use deno_core::Extension;
use deno_core::ModuleCodeString;
use deno_core::ModuleSpecifier;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

use crate::SNAPSHOT;

use crate::Entry;
use crate::deno_current_thread;

#[derive(Default)]
pub struct DenoContextOptions {
  pub entry: Option<Entry>,
  pub extensions: Option<fn() -> Vec<Extension>>,
}

pub struct DenoResponse<T, U> {
  rx: Option<oneshot::Receiver<Result<T, U>>>,
}

impl<T, U> DenoResponse<T, U> {
  pub fn recv(&mut self) -> Option<Result<T, U>> {
    let Some(rx) = self.rx.take() else {
      return None;
    };
    let Ok(response) = rx.blocking_recv() else {
      return None;
    };
    return Some(response);
  }
}

pub struct DenoContext<'a> {
  handle: Option<JoinHandle<()>>,
  sender: Option<std::sync::mpsc::Sender<DenoRequest<'a>>>,
}

pub enum DenoRequest<'a> {
  RunFile(oneshot::Sender<Result<(), ()>>, PathBuf),
  RunCode(oneshot::Sender<Result<(), ()>>, &'a str, &'a str),
}

impl DenoContext<'static> {
  pub fn new(options: DenoContextOptions) -> DenoContext<'static> {
    let mut deno_context = DenoContext{
      handle: None,
      sender: None,
    };

    let (tx, rx) = std::sync::mpsc::channel::<DenoRequest>();

    let handle = std::thread::spawn(move || {
      deno_current_thread(async move {
        let fs = std::sync::Arc::new(deno_fs::RealFs);
        let permissions = PermissionsContainer::allow_all();
      
        let bootstrap_options = BootstrapOptions {
          has_node_modules_dir: true,
          ..Default::default()
        };
      
        let worker_options = WorkerOptions {
          bootstrap: bootstrap_options.clone(),
          startup_snapshot: Some(SNAPSHOT),
          fs: fs.clone(),
          create_web_worker_cb: Arc::new(|_| panic!()),
          format_js_error_fn: Some(Arc::new(format_js_error)),
          ..Default::default()
        };
      
        let exe_path = std::env::current_exe().unwrap();
        let main_module = ModuleSpecifier::from_file_path(exe_path).unwrap();
        let mut main_worker = MainWorker::from_options(main_module.clone(), permissions, worker_options);
        main_worker.bootstrap(bootstrap_options.clone());

        // Incoming messages
        while let Ok(req) = rx.recv() {
          match req {
            DenoRequest::RunFile(_, _) => todo!(),
            DenoRequest::RunCode(tx, script_name, code) => {
              if let Ok(result) = main_worker.execute_script(script_name, ModuleCodeString::from_static(code)) {
                tx.send(Ok(()));
              } else {
                tx.send(Err(())).unwrap();
              }
            },
          }
        };

      });
    });

    deno_context.handle = Some(handle);
    deno_context.sender = Some(tx);
    return deno_context;
  }

  pub fn run_file() {}

  pub fn run_code(&self, script_name: &'static str, code: &'static str) -> DenoResponse<(), ()> {
    let (tx, rx) = oneshot::channel::<Result<(),()>>();
    self.send(DenoRequest::RunCode(tx, script_name, code)).unwrap();
    return DenoResponse { 
      rx: Some(rx) 
    };
  }

  pub fn join(&mut self) -> Result<(), ()> {
    let Some(handle) = self.handle.take() else {
      return Err(());
    };
    let Ok(result) = handle.join() else {
      return Err(());
    };
    return Ok(result);
  }

  fn send(&self, req: DenoRequest<'static>) -> Result<(), ()> {
    let Some(sender) = &self.sender else {
      return Err(());
    };
    let Ok(_) = &sender.send(req) else {
      return Err(());
    };
    return Ok(());
  }
}

/*

let exe_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
let main_module = Url::from_file_path(exe_path).unwrap();

let main_module_path = PathBuf::from("/home/dalsh/Development/deno-sandbox/deno1/pass-no-snapshot/pkg/index.js");
let main_module = Url::from_file_path(&main_module_path).unwrap();
*/