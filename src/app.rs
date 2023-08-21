use std::rc::Rc;
use std::{
    ops::ControlFlow,
    sync::{Arc, Mutex},
};

use ehttp::*;

#[derive(Debug)]
enum Download {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}


fn response_vec(ui: &mut egui::Ui, response: &ehttp::Response, test_vec :&mut Vec<String>) {

    if let Some(text) = response.text() {
        let mut text = response.text().unwrap();
        test_vec.push(text.to_string());
       } else {

        println!("Not working");

       }




    for i in test_vec{
        ui.label(i.to_string());


    }
    let text = response.text();

    
    

}

use egui::{TextBuffer, Response};
//use tokio::sync::watch::{self, Sender, Receiver};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    test_vec: Vec<String>,
    test_var: String,
    test_boo1: bool,
    url: String,
    #[serde(skip)]
    download: Arc<Mutex<Download>>,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            url: "http://localhost:8080/api/healthchecker".to_owned(),
            value: 2.7,
            test_var:format!("This test is not working"),
            test_vec:Vec::new(),
            test_boo1: false,
            download:Arc::new(Mutex::new(Download::None)),

        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
   // fn save(&mut self, storage: &mut dyn eframe::Storage) {
   //     eframe::set_value(storage, eframe::APP_KEY, self);
   // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, 
            value ,
            test_vec,
            test_var,
            test_boo1,
            url,
            download,

        
        
        
        
        } = self;


        

        

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let download: &Download = &self.download.lock().unwrap();


            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.label(format!("Here is some sexy text"));
            ui.separator();
            if ui.button("Request?").clicked() {
                let request = ehttp::Request::get(&self.url);
                match download {
                    Download::None => {}
                    Download::InProgress => {
                        println!("Wait for it…");
                    }
                
                    
                    Download::Done(response) => match response {
                        Err(err) => {
                           println!("{}",err);
                        }
                        Ok(response) => {
                            
                            println!("Does it work? {:?}", response_vec(ui, response, test_vec));
                        }
                    }
                }
                let download_store = self.download.clone();
                *download_store.lock().unwrap() = Download::InProgress;
                let egui_ctx = ctx.clone();
        
                ehttp::fetch(request, move |response| {
                    *download_store.lock().unwrap() = Download::Done(response);
                    egui_ctx.request_repaint(); // Wake up UI thread
                });
                

                
        
                
                //test_vec.push(tmp);
            
            }
            if ui.button("Un-Request").clicked() {
                test_vec.pop();
            }
            


        });

        }
    


 







    }



