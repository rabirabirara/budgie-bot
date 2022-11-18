import os

import asyncio
import discord
import youtube_dl

from discord.ext import commands
from dotenv import dotenv_values

env = dotenv_values(".env")
description = "fuck this bot"

class Budgie(commands.Cog):
    def __init__(self, bot):
        self.bot = bot
        self._conch_holder = None

    @commands.command()
    async def join(self, ctx, arg=None):
        if arg is not None:
            vcs = ctx.guild.voice_channels
            matches = []
            for vc in vcs:
                if vc.name == arg:
                    matches.append(vc)
            if len(matches) == 1:
                channel = matches[0]
            else:
                channel = ctx.author.voice.channel
                # await ctx.send("Choose from the following:")
                # TODO: show each differing voice channel and give the options, number of people in them, etc.
        else:
            voice = ctx.author.voice
            if voice is None:
                return await ctx.send("You must be in a channel for me to join it!")
            channel = voice.channel

        if ctx.voice_client is None:
            channel = self.bot.get_channel(channel.id)
            await channel.connect()
        else:
            await ctx.voice_client.move_to(channel)

    @commands.command()
    async def leave(self, ctx):
        if ctx.voice_client is None:
            # say i'm not in a channel!
            await ctx.send("I'm not in a voice channel right now anyway.")
        else:
            await ctx.voice_client.disconnect()

    @commands.command()
    async def say(self, ctx, msg):
        await ctx.send(msg)


intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix='%', description=description, intents=intents)

@bot.event
async def on_ready():
    print(f'We have logged in as {bot.user} (ID: {bot.user.id})')

async def main():
    async with bot:
        await bot.add_cog(Budgie(bot))
        await bot.start(env['DISCORD_TOKEN'])

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass

# # Suppress noise about console usage from errors
# youtube_dl.utils.bug_reports_message = lambda: ''
# 
# 
# ytdl_format_options = {
#     'format': 'bestaudio/best',
#     'outtmpl': '%(extractor)s-%(id)s-%(title)s.%(ext)s',
#     'restrictfilenames': True,
#     'noplaylist': True,
#     'nocheckcertificate': True,
#     'ignoreerrors': False,
#     'logtostderr': False,
#     'quiet': True,
#     'no_warnings': True,
#     'default_search': 'auto',
#     'source_address': '0.0.0.0',  # bind to ipv4 since ipv6 addresses cause issues sometimes
# }
# 
# ffmpeg_options = {
#     'options': '-vn',
# }
# 
# ytdl = youtube_dl.YoutubeDL(ytdl_format_options)
# 
# 
# class YTDLSource(discord.PCMVolumeTransformer):
#     def __init__(self, source, *, data, volume=0.5):
#         super().__init__(source, volume)
# 
#         self.data = data
# 
#         self.title = data.get('title')
#         self.url = data.get('url')
# 
#     @classmethod
#     async def from_url(cls, url, *, loop=None, stream=False):
#         loop = loop or asyncio.get_event_loop()
#         data = await loop.run_in_executor(None, lambda: ytdl.extract_info(url, download=not stream))
# 
#         if 'entries' in data:
#             # take first item from a playlist
#             data = data['entries'][0]
# 
#         filename = data['url'] if stream else ytdl.prepare_filename(data)
#         return cls(discord.FFmpegPCMAudio(filename, **ffmpeg_options), data=data)
# 
# 
# class Music(commands.Cog):
#     def __init__(self, bot):
#         self.bot = bot
# 
#     @commands.command()
#     async def join(self, ctx, *, channel: discord.VoiceChannel):
#         """Joins a voice channel"""
# 
#         if ctx.voice_client is not None:
#             return await ctx.voice_client.move_to(channel)
# 
#         await channel.connect()
# 
#     @commands.command()
#     async def play(self, ctx, *, query):
#         """Plays a file from the local filesystem"""
# 
#         source = discord.PCMVolumeTransformer(discord.FFmpegPCMAudio(query))
#         ctx.voice_client.play(source, after=lambda e: print(f'Player error: {e}') if e else None)
# 
#         await ctx.send(f'Now playing: {query}')
# 
#     @commands.command()
#     async def yt(self, ctx, *, url):
#         """Plays from a url (almost anything youtube_dl supports)"""
# 
#         async with ctx.typing():
#             player = await YTDLSource.from_url(url, loop=self.bot.loop)
#             ctx.voice_client.play(player, after=lambda e: print(f'Player error: {e}') if e else None)
# 
#         await ctx.send(f'Now playing: {player.title}')
# 
#     @commands.command()
#     async def stream(self, ctx, *, url):
#         """Streams from a url (same as yt, but doesn't predownload)"""
# 
#         async with ctx.typing():
#             player = await YTDLSource.from_url(url, loop=self.bot.loop, stream=True)
#             ctx.voice_client.play(player, after=lambda e: print(f'Player error: {e}') if e else None)
# 
#         await ctx.send(f'Now playing: {player.title}')
# 
#     @commands.command()
#     async def volume(self, ctx, volume: int):
#         """Changes the player's volume"""
# 
#         if ctx.voice_client is None:
#             return await ctx.send("Not connected to a voice channel.")
# 
#         ctx.voice_client.source.volume = volume / 100
#         await ctx.send(f"Changed volume to {volume}%")
# 
#     @commands.command()
#     async def stop(self, ctx):
#         """Stops and disconnects the bot from voice"""
# 
#         await ctx.voice_client.disconnect()
# 
#     @play.before_invoke
#     @yt.before_invoke
#     @stream.before_invoke
#     async def ensure_voice(self, ctx):
#         if ctx.voice_client is None:
#             if ctx.author.voice:
#                 print("connecting to {ctx.author.voice.channel}")
#                 await ctx.author.voice.channel.connect()
#                 print("connected")
#             else:
#                 await ctx.send("You are not connected to a voice channel.")
#                 raise commands.CommandError("Author not connected to a voice channel.")
#         elif ctx.voice_client.is_playing():
#             ctx.voice_client.stop()
# 
# 
# intents = discord.Intents.default()
# intents.message_content = True
# 
# bot = commands.Bot(
#     command_prefix=commands.when_mentioned_or("!"),
#     description='Relatively simple music bot example',
#     intents=intents,
# )
# 
# 
# @bot.event
# async def on_ready():
#     print(f'Logged in as {bot.user} (ID: {bot.user.id})')
#     print('------')
# 
# 
# async def main():
#     async with bot:
#         await bot.add_cog(Music(bot))
#         try:
#             await bot.start(env['DISCORD_TOKEN'])
#         except Exception as e:
#             print(e)
# 
# 
# asyncio.run(main())

