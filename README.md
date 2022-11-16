# budgie-bot
A text-to-speech parakeet.

# Motivation
A simple bot that does one thing: It will parrot everything you say in a voice channel for you.

Ever been in a voice call, muted, but still wanting to join the conversation?  Typing just doesn't do it - who looks at chat anyway?

Your friends could be playing games with each other, joking around, and you will be missing out with your sad, desperate sentences sent by text
in the general chat.

Well, now you can annoy them using a Text-to-Speech voice.

# How to use

For now, you must be in a voice channel.  Pretend as though the bot is speaking for you.  You cannot use the bot from a distance *yet*.

Ask the bot to join your voice channel using `!join`.  Then, use either `!say` to say one line with TTS, or `!parrot` to toggle on/off automatic TTS
on any messages you send.

`!parrot` is like a mutex.  The first user to call `!parrot` is the user who has sole rights over the bot's functionality.  Play nice!

# Ways of improving this bot

- add permissions and user detection: only one user can use parrot at a time (first come first serve), and all other users' messages should be ignored
- let users use the bot even while not in call, so they can be in one call but pester another
- add the use of different voices
- add a settings framework (database) that allows users to set their voice profile, i.e. voice, rate, pitch
- stop using google TTS and use a real TTS library, with a respectable voice; if higher quality TTS is slower, give users the options to choose either in their voice profile
