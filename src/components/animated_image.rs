use yew::prelude::*;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Collections {
    Punk,
    CyberBroker,
    Captain
}

#[derive(PartialEq, Properties, Clone)]
pub struct AnimatedImageProps {
    pub collection: String,
}

struct ItemInfo {
    name: String,
    id: String 
}

#[function_component(AnimatedImage)]
pub fn animated_image(props: &AnimatedImageProps) -> Html {

    let i = match props.collection.as_str() {
         "punk" => {
            ItemInfo {
                name: String::from("CryptoPunk"),
                id: String::from("#0069")
            }
         }
         "broker" => {
             ItemInfo {
                name: String::from("CyberBroker"),
                id: String::from("#0069")
             }
         }
         "captain" => {
             ItemInfo {
                name: String::from("Captainz"),
                id: String::from("#6669")
             }
         }
         _ => {
             ItemInfo {
                name: String::from("Unknown"),
                id: String::from("I Wish")
             }

         }
    };
    html! {
        <div class={"animated-image"} id={props.collection.clone()}>
            <div class={"collection-name"}>{i.name}</div>
            <div class={"token-id"}>{i.id}</div>
        </div>
    }

}
