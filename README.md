# Letterbox
A simplistic, lightweight ModMail ~~bot~~ app for Discord using modern features like forums.

## Philosophy
Advanced features like a web panel are not the goal here, nor is making the configuration flexible enough to satisfy absolutely everyone's needs.

Only posting to a forum will ever be supported; there are several reasons why:
- Text channels can be viewed without permission with the right tools since Discord API notoriously leaks information such as channel name and topic - which is not normally a big deal but ask youself if you really want this information to be effectively public? Or otherwise have completely meaningless names which make it hard to identify open threads?
- Text channels take up a lot of room - you can collapse or hide them but then they're harder to get to. Even if you're okay with that, you can only have 500 non-thread channels in a Discord server. This isn't much of an issue if the channel is deleted when it's closed - but now either you lose all your message history or searching it is less convinient or not possible.
- Text channels have less organisation features - no tags, no putting spaces in names, no searching the name or topic. You can't mark them as closed either - not that it matters since nobody uses them as an archive (I hope) for reasons mentioned in the previous point.

Hopefully this has convinced you. Sorry if this sounded like a rant. The truth is probably that almost every bot making use of dynamic text channels were probably made before threads/forums. Though there is a downside. @everyone and @here pings do not work.

Some good features to include in the future could be:
- Requiring confirmation before threads are created.
- Supporting multiple different roles (not just Staff) which can have different names which appear in embeds and more specific permissions.

## Features
- Users can DM the bot to create a thread.
- More messages will add to the thread.
- These threads appear tagged as "Open" in a forum channel.
- Staff can reply using the reply command.
- Staff can *anonymously* reply using anon_reply.
- Messages can be edited and deleted using their respective commands.
- Threads can be closed using the close command. This will lock the forum post and retag it as "Closed".
- anon_close and silent_close are available. silent_close will not attempt to notify the user that the thread has been closed.
- Staff can open a thread to a user without their interaction using the contact command.