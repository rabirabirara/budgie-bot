import os
import discord
from discord.ext import commands
from dotenv import dotenv_values

env = dotenv_values(".env")

description = """
    An example bot.
"""

intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix='%', description=description, intents=intents)
# client = discord.Client(intents=intents)

@bot.event
async def on_ready():
    print(f'We have logged in as {bot.user} (ID: {bot.user.id})')
    
@bot.event
async def on_message(msg):
    print(msg.content)

@bot.command()
async def say(ctx, msg: str):
    await ctx.send(msg)
    
@bot.group()
async def cool(ctx):
    """Says if a user is cool.
    In reality this just checks if a subcommand is being invoked.
    """
    if ctx.invoked_subcommand is None:
        await ctx.send(f'No, {ctx.subcommand_passed} is not cool')

@cool.command(name='Spencer')
async def _bot(ctx):
    """Is the bot cool?"""
    await ctx.send('Yes, the bot is cool.')
    
bot.run(env['DISCORD_TOKEN'])