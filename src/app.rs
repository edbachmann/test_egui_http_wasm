use std::rc::Rc;
use std::{
    ops::ControlFlow,
    sync::{Arc, Mutex},
};
use serde_json::Value;
use serde_json::from_str;
use ehttp::*;

#[derive(Debug)]
enum Download {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

fn make_request(download:Arc<Mutex<Download>>, url: String, ui:  &egui::Context, test_vec :&mut Vec<String>) {

    

    let download_store = download.clone();
   // *download_store.lock().unwrap() = Download::InProgress;
    let egui_ctx = ui.clone();

    let request = ehttp::Request::get(url);

    ehttp::fetch(request, move |response| {
        *download_store.lock().unwrap() = Download::Done(response);
        egui_ctx.request_repaint(); // Wake up UI thread
    });

   



}

fn response_vec(ui: &mut egui::Ui, response: &ehttp::Response, test_vec :&mut Vec<String>) {

    if let Some(text) = response.text() {
        let mut text = response.text().unwrap();
       let v: Value = serde_json::from_str(text).unwrap();
      // text = v["message"];
        test_vec.push(v["message"].to_string());
       } else {

        println!("Not working");

       }





    let text = response.text();

    
    

}

use egui::{TextBuffer, Response, Ui};
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
    download1: Arc<Mutex<Download>>,
    #[serde(skip)]
    download2: Arc<Mutex<Download>>,
    #[serde(skip)]
    download3: Arc<Mutex<Download>>,
    #[serde(skip)]
    download4: Arc<Mutex<Download>>,
    #[serde(skip)]
    download5: Arc<Mutex<Download>>,

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
            download1:Arc::new(Mutex::new(Download::None)),
            download2:Arc::new(Mutex::new(Download::None)),
            download3:Arc::new(Mutex::new(Download::None)),
            download4:Arc::new(Mutex::new(Download::None)),
            download5:Arc::new(Mutex::new(Download::None)),

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
            download1,
            download2,
            download3,
            download4,
            download5,

        
        
        
        
        } = self;


        

        

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's


            
         




            ui.label(format!("Here is some sexy text"));
            ui.separator();
            ui.horizontal( |ui| {
            if ui.button("Request line 1").clicked() {
                let tmp: &Download = &self.download1.lock().unwrap();

                match tmp {
                    Download::None => {}
                    Download::InProgress => {
                        println!("Wait for it…");
                    }
                
                    
                    Download::Done(response) => match response {
                        Err(err) => {
                           println!("{}",err);
                        }
                        Ok(response) => {
                            
                            println!("Does it work? {:?}", response_vec(ui, &response, test_vec));
                        }
                    }
                }

                make_request(self.download1.clone(), url.clone(), ctx, test_vec)
                

                
          
                
                //test_vec.push(tmp);
            
            }

            if ui.button("Request line 2").clicked() {
                let tmp: &Download = &self.download2.lock().unwrap();

                match tmp {
                    Download::None => {}
                    Download::InProgress => {
                        println!("Wait for it…");
                    }
                
                    
                    Download::Done(response) => match response {
                        Err(err) => {
                           println!("{}",err);
                        }
                        Ok(response) => {
                            
                            println!("Does it work? {:?}", response_vec(ui, &response, test_vec));
                        }
                    }
                }

                make_request(self.download2.clone(), url.clone(), ctx, test_vec)
                

                
          
                
                //test_vec.push(tmp);
            
            }
            if ui.button("Request line 3").clicked() {
                let tmp: &Download = &self.download3.lock().unwrap();

                match tmp {
                    Download::None => {}
                    Download::InProgress => {
                        println!("Wait for it…");
                    }
                
                    
                    Download::Done(response) => match response {
                        Err(err) => {
                           println!("{}",err);
                        }
                        Ok(response) => {
                            
                            println!("Does it work? {:?}", response_vec(ui, &response, test_vec));
                        }
                    }
                }

                make_request(self.download3.clone(), url.clone(), ctx, test_vec)
                

                
          
                
                //test_vec.push(tmp);
            
            }
    });
            if ui.button("Un-Request").clicked() {
                test_vec.pop();
            }
            
            for i in test_vec{
                ui.label(i.to_string());
        
        
            }


        });

        }
    


 







    }



