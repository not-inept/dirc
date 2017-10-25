#DIRC
DIRC is a Discord bot that turns a guild into an IRC client.

#About
I'm new to Rust and have been having difficulties with my WeeChat Relay, so I
instead would like to use Discord, which I already use for other communications,
as my IRC client as well. Although the final result of this project may actually
be useful to me, I'm really doing it to build up some experience with Rust.

If you'd like, I would really appreciate you taking a look through my code and
pointing out all the dumb things I do so I can work towards something that's
more idiomatic as I figure out how everything works.

#Status
Overall: Nonfunctional

Currently you can connect to a server using `~connect <server> <nick> <comma separated channels>`
Sending messages from discord does not work, and the messages themselves include the full information from the IRC client and not exclusively the message content.

# General Plan/features:
[ ] Be able to send a message to a specific channel
[X] Be able to create a channel category
[X] Be able to create a channel in a specific category
[X] Be able to connect to server
[ ] Be able to join channel on connected server
[ ] Be able to receive messages/print messages
[ ] Be able to list channels
[ ] Start and stop commands that start/stop the irc server for the user
[ ] ability to associate a channel/guild with a user
[X] Connect to a server and create a category for that server, with a default #server channel
[ ] Create a default dirc-log channel
[ ] Join that creates a discord channel and links it to the corresponding irc one
[ ] change user name to IRC user's name per message

[ ] Multiple Guilds
  [ ] Have separate discord monitor for a launch server.
  [ ] Ability to create a guild and ensure self has proper permissions.
  [ ] Be able to securely support multiple users/guilds.
