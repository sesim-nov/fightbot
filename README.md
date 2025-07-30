# fightbot
Simple bot for queuing 2v2-4v4's with minimal permissions

This bot handles differently sized queues without needing multiple channels, and is open source so you can see what's being added to your server.

It uses "?" as the trigger with the following possible commands:
 - ?r
 - ?start
 - ?cancel

Each of these must be used with a slash command:
 - /2v2
 - /3v3
 - /4v4
 - /me (used only for ?cancel)

?r /[group size] registers the user for the group size selected via slash command.
When a group size has reached the maximum number of users, the bot will automatically comment in the channel with a randomized team selection.

Example use case: ?r /3v3
 - User gets registered for 3v3 queue.

?start /[group size] will force completion of the queue regardless of user count

Example use case: ?start /4v4 (issued when only 7 people have queued)
 - Bot returns randomized team list with one team of 4 and one team of 3

?cancel clears the queue, either of the user calling the command or of the entire group

Example use case: ?cancel /2v2
 - 2v2 queue is cleared so the bot doesn't get confused with old users next time a group of users queues that group size

Example use case 2: ?cancel /me
 - User is removed from whichever group they were queued to
