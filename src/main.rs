#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate irc;
extern crate config;

use std::collections::HashMap;
use config::File;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use std::env;

use serenity::prelude::*;
use serenity::model::*;
use serenity::utils::*;
use serenity::model::{ChannelType, GuildId};
use std::sync::{Arc, RwLock};
use serenity::http::create_channel;
use std::default::Default;
use irc::client::prelude::*;

struct Handler;

fn create_category(guild : GuildId, name : String) -> GuildChannel {
    let category = create_channel(guild.0, &json!({
        "type": ChannelType::Category as i64,
        "name": name,
        "position": 1,
    }));
    return category.unwrap();
}

fn create_channel_in_category(guild : GuildId, parentId : u64, name : String) -> GuildChannel {
    let channel = create_channel(guild.0, &json!({
        "type": ChannelType::Text as i64,
        "name": name,
        "position": 1,
        "parent_id": parentId,
    }));

    return channel.unwrap()
}

fn clean_up_guild(guild : GuildId) {
    let channels : HashMap<ChannelId, GuildChannel> = guild.channels().unwrap();
    for (_, channel) in &channels {
        let _res = channel.delete();
    }
    let res = guild.create_channel("dirc", ChannelType::Text).unwrap();
    res.say("The guild has been cleaned.");
}

fn join_channel(guild : GuildId, channel : String, server : IrcServer) {
    // check if channel exists in guild -> server category
    // if not, create channel
    // bind relay
}

fn clean_discord_name(name : String) -> String {
    let hs = '#'.to_string();
    let at = '@'.to_string();
    let cleaned_name = name.chars().filter(|v|
        v.to_string() != hs && v.to_string() != at
    ).collect();
    return cleaned_name;
}

fn connect_server(guild : GuildId, serverName : String, nick : String, chans : Vec<String>) {
    let cleaned_name = clean_discord_name(serverName.clone());
    println!("Clean name: {}|", cleaned_name);
    let category = create_category(guild, cleaned_name);
    println!("Nick: {}|", nick);
    println!("Server: {}|", serverName);
    println!("{}|", chans[0]);

    let cfg = Config {
        nickname: Some(nick),
        server: Some(serverName),
        channels: Some(chans),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();

    server.identify().unwrap();
    server.for_each_incoming(|message| {
        let rnick = message.source_nickname().unwrap_or("SERVER");
        let target = message.response_target().unwrap_or("SERVER");
        let target_clean = clean_discord_name(String::from(target));
        let tags = message.tags.to_owned().unwrap_or(Vec::new());
        let prefix = message.prefix.to_owned().unwrap_or(String::from("SERVER"));
        let command = message.command.to_owned();
        let channels : HashMap<ChannelId, GuildChannel> = guild.channels().unwrap();
        let mut found = false;

        for (channel, guild_channel) in &channels {
            let cat : u64 = match guild_channel.category_id {
                Some(x) => x.0,
                None => 0
            };

            if guild_channel.name.to_lowercase() == target_clean.to_lowercase()
                && cat == category.id.0 {
                let res3 = channel.say(message.to_string());
                found = true;
            }
        }
        if !found {
            let res4 = create_channel_in_category(guild, category.id.0, target_clean);
            res4.say(&message.to_string());
        }

    }).unwrap()
}

impl EventHandler for Handler {
    fn on_message(&self, context : Context, msg: serenity::model::Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say("Pong!") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn on_ready(&self, context : Context, ready: Ready) {
        let res = context.edit_profile(|profile| {
            profile.username("Dirc")
        });
    }
}

fn main() {
    // Load settings (right now just discord, later more?)
    let mut settings = config::Config::default();
    settings
        .merge(File::with_name("conf/discord.toml")).unwrap();
    let discord = settings.get::< HashMap<String, String>>("discord").unwrap();

    // Login with a bot token
    let mut client = Client::new(&discord["token"], Handler);
    println!("Chan name: {} {}", ChannelType::Text.name(), ChannelType::Text as i64);
    println!("Chan name: {} {}", ChannelType::Category.name(), ChannelType::Category as i64);
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .on("clean", clean)
        .on("connect", connect)
        .on("join", join)
        .on("msg", privmsg)
    );
    let _ = client.start();
}

command!(privmsg(_context, msg, args) {
    let target = args.single::<String>().unwrap();
    let content = args.join(" ");
    println!("Context: {}", msg.channel_id);
    println!("Target: {}", target);
    println!("Content: {}", content);
});

command!(join(_context, msg, args) {
    let channel = args.single::<String>().unwrap();
});

command!(connect(_context, msg, args) {
    let guild : GuildId = msg.guild_id().unwrap();
    let server : String = args.single::<String>().unwrap();
    let nick : String = args.single::<String>().unwrap();
    let channels_string : String = args.single::<String>().unwrap();
    let channels_split = channels_string.split(",");
    let channels : Vec<String> = channels_split
        .map(|s| String::from("#") + s)
        .collect();
    connect_server(guild, server, nick, channels);
});

command!(clean(_context, msg) {
    clean_up_guild(msg.guild_id().unwrap())
});
