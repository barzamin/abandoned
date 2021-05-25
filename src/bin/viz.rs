#![feature(proc_macro_hygiene)]

use maud::{html, Render, DOCTYPE};
use osmpbfreader::{Node, OsmId, OsmObj};
use ron::ser::PrettyConfig;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    input: String,
    output: String,
}

fn osm_viewer_url(obj: &OsmObj) -> String {
    match obj.id() {
        OsmId::Node(id) => format!("https://www.openstreetmap.org/node/{}", id.0),
        OsmId::Way(id) => format!("https://www.openstreetmap.org/way/{}", id.0),
        OsmId::Relation(id) => format!("https://www.openstreetmap.org/relation/{}", id.0),
    }
}

fn gmaps_url(node: &Node) -> String {
    format!(
        "https://www.google.com/maps/place/{},{}",
        node.lat(),
        node.lon()
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opts::from_args();
    let file_contents = std::fs::read(opt.input)?;
    let abandoned: Vec<OsmObj> = ron::de::from_bytes(&file_contents)?;

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title {"abandoned"}
                style {r#"
                    dt {font-weight: bold;}
                "#}
            }
            body {
                @for obj in &abandoned {
                    h2 {
                        "Abandoned "
                        @match obj {
                            OsmObj::Node(_) => "node",
                            OsmObj::Way(_) => "way",
                            OsmObj::Relation(_) => "rel",
                        }
                        " ["
                        @if let Some(name) = obj.tags().get("name") {
                            (name)
                        } @else {
                            "nameless"
                        }
                        "]"
                    }
                    a href=(osm_viewer_url(&obj)) {"view in OSM"}
                    // @match obj {
                    //     OsmObj::Node(node) => {a href=(gmaps_url(node)) {"view in gmaps"}}
                    //     Way::Node()
                    // }
                    dl {
                        @for (k, v) in obj.tags().iter() {
                            dt { (k) }
                            dd { (v) }
                        }
                    }
                    details {
                        summary {"Raw internal data"}
                        pre {(
                            ron::ser::to_string_pretty(obj,
                                PrettyConfig {..Default::default()})?
                        )}
                    }
                }
            }
        }
    };

    let mut w = File::create(opt.output)?;
    w.write_all(markup.render().into_string().as_bytes())?;

    Ok(())
}
