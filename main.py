import os

import asyncio
import discord
import signal

from discord.ext import commands
from dotenv import dotenv_values

env = dotenv_values(".env")

description = """
    An example bot.
"""

class Budgie(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.command()
    async def join(self, ctx, *, channel: discord.VoiceChannel):
        """Joins a voice channel"""
        if ctx.voice_client is not None:
            return await ctx.voice_client.move_to(channel)
        await channel.connect()

    @commands.command()
    async def leave(self, ctx):
        """Leaves the current voice channel, if in one"""
        if ctx.voice_client is None:
            # say i'm not in a channel!
            ctx.send("I'm not in a voice channel right now anyway.")
        else:
            await ctx.voice_client.disconnect()

    @commands.command()
    async def say(self, ctx, msg):
        await ctx.send(msg.content)


# @bot.group()
# async def cool(ctx):
#     """Says if a user is cool.
#     In reality this just checks if a subcommand is being invoked.
#     """
#     if ctx.invoked_subcommand is None:
#         await ctx.send(f'No, {ctx.subcommand_passed} is not cool')
# 
# @cool.command(name='Spencer')
# async def _bot(ctx):
#     """Is the bot cool?"""
#     await ctx.send('Yes, the bot is cool.')
    
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

