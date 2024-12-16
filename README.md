# Letterbox
A simplistic, lightweight ModMail ~~bot~~ app for Discord using modern features like forums.

## Philosophy
Advanced features like a web panel are not the goal here, nor is making the configuration flexible enough to satisfy absolutely everyone's needs.

Only posting to a forum will ever be supported - hidden channel viewers exist but hidden thread viewers do not, and text channels need to be deleted when a thread is closed otherwise there will be a lot of clutter and the 500 channel limit could easily be reached on a large server.

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