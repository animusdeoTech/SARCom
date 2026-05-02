# Blueprint: Claude Code Projects

> **How to use this file**: At the start of a new project, tell Claude:
> "Read @Blueprint.md, then walk me through the setup."
> Claude will go through each phase with you step by step, generating two files:
> 1. **CLAUDE.md** — project instructions Claude reads every session
> 2. **Instructions.md** — your personal reference for using Claude Code in this project

### Key Principle
**Build less, use more.** A basic CLAUDE.md + one or two slash commands that you actually use every session will produce more value than a comprehensive system you never touch. Start minimal, add tools only when you feel friction.

---

# Phase 0: Set Up the Project Folder

Create the project folder in the Obsidian vault and initialize the Claude Code structure:

```
<Project Name>/
├── CLAUDE.md
├── Instructions.md
└── .claude/
    ├── commands/           ← slash commands (add when needed)
    └── rules/              ← modular rules (add when needed)
```

**Claude should:**
1. Ask the user where in the vault this project should live
2. Ask the user what kind of project this is (see Project Archetypes below for examples)
3. Create the folder structure
4. Move to Phase 1

Don't create output subfolders in advance — let them emerge from actual use.

### Why `.claude/rules/`?

Rules are markdown files that Claude loads **only when relevant** — based on glob patterns or context. This keeps your CLAUDE.md lean (universal instructions only) while still giving Claude detailed guidance when it needs it. Example: a rule in `rules/journaling.md` only loads during journaling sessions, not when you're organizing files.

---

# Phase 1: Build the CLAUDE.md Together

Enter **Plan mode** (`Shift+Tab` twice) to draft the CLAUDE.md without writing files yet.

**Claude should walk through each section of the CLAUDE.md template below, asking the user to fill in the placeholders.** Don't dump the whole template at once — go section by section:

1. **Project Name & Purpose**: "What is this project? One sentence: what do you want Claude to help you with?"
2. **Role**: "What kind of assistant do you need? Be specific — 'personal development coach who asks tough questions' works better than 'helpful assistant'."
3. **Scope**: "What falls inside this project? What's out of bounds?"
4. **Recurring Tasks**: "What will you ask me to do most often? (These become slash commands later.)"
5. **Example Exchange**: "Show me what a great interaction looks like — and what a bad one looks like. Walk me through one round of each so I know what to aim for and what to avoid."
6. **Key References** *(if applicable)*: "Are there documents, systems, or templates I should know about? Don't paste them here — just tell me what they are and where to find them."
7. **Connected Tools** *(if applicable)*: "Does this project involve external tools? (Calendar, email, apps, websites, specific files)"

When all sections are filled in, switch to **Normal mode** and save the CLAUDE.md.

---

## CLAUDE.md Design Principles

Before the template, understand **why** it's structured this way. These principles draw from Anthropic's prompt engineering documentation, their context engineering research, community best practices for CLAUDE.md files, and peer-reviewed prompting research (see: Schulhoff et al. 2024, "The Prompt Report"; Wei et al. 2022, "Chain-of-Thought Prompting").

### 1. Every word costs attention

CLAUDE.md loads into every single session. It competes for space in the context window with your conversation, file contents, and tool outputs. Research shows that as instruction count rises, instruction-following quality drops — not just for new instructions, but **uniformly across all of them**. Claude Code's own system prompt already uses many instructions. Your CLAUDE.md should add as few as possible on top of that.

**Target: under 150 lines.** If it grows beyond that, move specialized content into `.claude/rules/` files or reference documents that Claude can read on demand.

### 2. Tell Claude what to do, not what to avoid

Claude generalizes better from positive instructions. Instead of "Don't give me long explanations," write "Keep explanations to 2-3 sentences unless I ask for more." Instead of "Don't use bullet points for everything," write "Use flowing prose paragraphs by default; reserve bullet points for lists of 4+ items."

When you do need a boundary, **pair it with an alternative and explain why** — Claude generalizes from reasoning, not just rules. "Always verify surprising claims with web search (because incorrect medical/financial/legal information has real consequences)" is stronger than "Always verify claims."

Note: while positive framing is the default, **negative examples** are a different story — they're extremely valuable (see Principle 3).

### 3. Show, don't tell — examples beat rules (including negative examples)

This is the highest-impact technique alongside clarity. Peer-reviewed research (Brown et al. 2020) and all major AI providers agree: 2-5 well-chosen examples teach Claude patterns more effectively than a page of written rules.

**Positive examples** show what good looks like. **Negative examples** define the boundaries — they show Claude where it should stop, what to avoid, and prevent over-triggering. Anthropic's own documentation calls negative examples "extremely important" for defining boundaries.

In your CLAUDE.md, the **Example Exchange** section includes both a positive example (ideal interaction) and a negative example (what to avoid). In slash commands, the **Output Example** section shows the desired output, and the **Edge Cases** section handles boundary conditions.

### 4. Your CLAUDE.md sets the tone

Claude mirrors the style and tone of its prompt. If your CLAUDE.md is written formally, Claude will default to a formal tone. If it's conversational, Claude will be conversational. This means the Example Exchange section doesn't just teach Claude *what* to do — it calibrates *how* Claude sounds when it does it.

Write your CLAUDE.md in the tone you want Claude to use with you.

### 5. Hit the right altitude

Anthropic's context engineering research identifies a "Goldilocks zone" for instructions: specific enough to guide behavior, flexible enough for Claude to adapt. Too rigid (hardcoded if/else rules for every scenario) creates brittleness — Claude can't handle anything the rules didn't anticipate. Too vague ("be helpful") gives Claude nothing to work with.

The sweet spot: give Claude **heuristics and principles** it can apply across situations, not step-by-step scripts. "Ask probing questions before giving answers (because active recall beats passive reading)" is better than "When the user asks a question, first ask them what they already know, then ask what they think the answer might be, then..."

### 6. Progressive disclosure — load what's needed, when it's needed

Don't stuff every possible instruction into CLAUDE.md. Instead, tell Claude **where to find** detailed information so it loads only what's relevant:
- Use `.claude/rules/` for context-specific instructions (glob patterns control when they load)
- Reference external files: "For quiz formatting rules, read `.claude/rules/quizzes.md`"
- Keep CLAUDE.md for universal, every-session instructions only

This mirrors how Anthropic builds their own Skills system: a small index file points to detailed instructions that only load when triggered.

### 7. Tell Claude how to verify its own work

Anthropic's best practices identify self-verification as the single highest-leverage addition to any prompt. If there's a way for Claude to check whether its output is correct — expected format, criteria to meet, a consistency check — state it explicitly. "After generating a study note, verify that every key term from the source material appears in the note" is more effective than hoping Claude remembers to be thorough.

---

## CLAUDE.md Template

```markdown
# [Project Name]

## Who I'm Helping
- **User**: [short description — enough so Claude knows your context]
- **Project**: [one-line purpose]
- **Tool**: Obsidian + Claude Code

## Role
You are a [specific role — e.g., "reflective journal coach who asks hard questions",
"medical tutor who teaches through Socratic questioning", "productivity system operator
who triages and executes"]. Your job is to:
1. [Primary task]
2. [Secondary task]
3. [Tertiary task]
(3-5 items max. More means the project scope is too broad — split it.)

## How to Interact
[Pick what applies. Delete the rest. Add your own. Explain WHY each matters.]
- Ask probing questions before giving answers (because active recall beats passive reading)
- Challenge incorrect reasoning directly (because uncorrected mistakes compound)
- Hold me accountable to commitments I've made (because consistency matters more than motivation)
- Use web search when unsure about a claim (because wrong information has real consequences)
- Flag spelling and grammar mistakes (I have dyslexia — correct patiently, never skip)
[The WHY in parentheses helps Claude generalize the principle to new situations.]

## Output Format
- Obsidian-compatible markdown with [[wikilinks]] (so notes interconnect in the graph)
- Bold key terms on first use (so scanning is fast during review)
- Keep explanations to 2-3 sentences unless I ask for depth
- Language: English by default

## Example Exchange
[Show Claude what a GOOD interaction looks like — and what a BAD one looks like.
The positive example calibrates tone and depth. The negative example defines the
boundary Claude should never cross. Replace these with examples that fit your project.]

<example type="positive">
User: "I'm reading about ventilation-perfusion mismatch. Can you explain it?"
Claude: "Before I explain — what do you already understand about how gas exchange
works in the alveoli? Walk me through it and I'll build from where you are."
User: [explains their understanding]
Claude: [corrects misconceptions, fills gaps, connects to clinical relevance,
asks a follow-up to test understanding]
</example>

<example type="negative">
User: "I'm reading about ventilation-perfusion mismatch. Can you explain it?"
Claude: "Ventilation-perfusion mismatch occurs when..." [launches into a full
explanation without asking what the user already knows, gives the answer instead
of drawing out understanding, doesn't test comprehension afterward]
WHY THIS IS WRONG: This bypasses active recall. The user reads passively and
learns nothing they couldn't get from a textbook.
</example>

[For non-study projects, the examples would look different:]

<example type="positive">
User: "I had a rough day. Let's journal."
Claude: "What's one moment from today that's still sitting with you?"
</example>

<example type="negative">
User: "I had a rough day. Let's journal."
Claude: "I'm sorry to hear that. Here are some journaling prompts: 1. What
happened today? 2. How did it make you feel? 3. What would you do differently?"
WHY THIS IS WRONG: This is a generic prompt dump. It doesn't ask a specific
question, doesn't create a conversational space, and doesn't draw on any
previous entries for continuity.
</example>

## Scope
[What this project covers — specific enough that Claude knows what belongs here
vs. in a different project.]

## Verification
[How Claude should check its own work. Examples:]
- After generating a study note: verify every learning objective is addressed
- After a journal prompt: check that the question is open-ended, not yes/no
- After organizing files: confirm no wikilinks are broken
[Delete this section if not applicable. But try to find something — it's high-leverage.]

## Corrections Log
[Document recurring mistakes here so they get fixed permanently. Start empty.
Add entries when you notice Claude repeating errors across sessions.]
- [e.g., "Claude tends to give the answer instead of asking me first — always
  lead with a question before explaining."]
- [e.g., "When formatting tables, Claude forgets the blank line before the table
  that Obsidian needs to render it."]

## Key References
[Don't paste content here. Point to where Claude can find it when needed.]
- Learning objectives: @learning-objectives.pdf
- Diagnostic criteria: @DSM-criteria.md
- Personal goals: @goals-2025.md
[Leave empty or delete if not applicable.]

## Connected Tools
[External services this project touches. Note access patterns.]
- Google Calendar: read events to plan study sessions
- Gmail: search for course announcements
[Leave empty or delete if not applicable.]

## Compaction Instructions
When compacting this conversation, always preserve:
- Decisions made and their reasoning
- Current task status and next steps
- Any commitments or deadlines mentioned
[Customize per project. This tells /compact what matters most.]

## Housekeeping
- When a session is renamed or a new topic session is started, update the Sessions Log in Instructions.md
- When creating a new slash command, save it as `.claude/commands/project_<command-name>.md` using lowercase with hyphens (e.g., `project_study-note.md` → invoked as `/project_study-note`)

## File Naming
Follow the global skill /organize-project for naming rules.
```

### What NOT to put in CLAUDE.md

- **User-reference material**: session logs, folder structure maps, how-to reminders — these are for *you*, not for Claude. Put them in Instructions.md where they don't eat context every session
- **Content that belongs in a reference file**: diagnostic criteria, formulas, templates — put these in separate files and reference them with `@filename`
- **Instructions for rare tasks**: if it only applies sometimes, put it in `.claude/rules/` or in a slash command
- **Style rules that an existing tool handles**: if Obsidian Linter handles formatting, don't duplicate those rules here
- **Copy-pasted content from sources**: link to it, don't embed it — context window space is precious
- **Long lists of "never do X"**: reframe as positive instructions with alternatives, or as negative examples with reasoning

---

# Phase 1.5: Scan Existing Materials and Generate Instructions.md

After the CLAUDE.md is saved, Claude should look at what's already in the project folder and generate the Instructions file.

**Claude should:**
1. List all files in the project directory (PDFs, images, notes, exports)
2. Summarize what's available: "You have X PDFs, Y notes, Z other files"
3. Ask: "Do you want me to organize these with `/organize-project`, or leave them as-is for now?"
4. If there are relevant documents (syllabi, exported data, templates), read them and suggest how they inform the project scope
5. **Generate Instructions.md from the template below**, customizing it for this project:
   - Fill in the **Project Folder Structure** with actual folder names, output subfolders, and file naming patterns for this project type
   - Add project-specific commands to the **Essential Commands Cheat Sheet** (e.g., a study project gets `/project_quiz`, a journaling project gets `/project_journal`)
   - Add the first session (`config`) to the **Sessions Log**
   - Review all sections critically — remove anything that doesn't apply to this project type, add anything project-specific that the user would want as a reference

---

## Instructions.md Template

The template below is the starting point. During project setup, Claude generates a customized version. Sections marked `[CUSTOMIZE]` get tailored to the project. Sections marked `[STANDARD]` stay as-is unless there's a reason to change them.

````markdown
# Claude Code — Reference Guide

---

## Starting a Session [STANDARD]

A **session** in Claude Code is the equivalent of a **chat** in the web app — it has its own message history and context window. Use `/rename` to give it a name (like titling a chat) so you can find it later with `/resume`.

```bash
cd "<project folder>"
claude                    # new session
claude -c                 # continue your last session
claude -r "session-name"  # resume a specific named session
```

Claude automatically reads `CLAUDE.md` from the project folder. You don't need to re-explain context each time.

---

## Project Folder Structure [CUSTOMIZE]

[Replace this entire section with the actual folder structure for this project.
Show the output folders, file naming patterns, and what each folder contains.]

```
<Project Name>/
├── CLAUDE.md
├── Instructions.md
├── .claude/
│   ├── commands/
│   │   └── project_<command-name>.md
│   └── rules/
│
├── [source materials]                     ← PDFs, articles, reference docs
│
└── [output folders]/                      ← Created by slash commands as you work
    └── <files named per command>
```

---

## Slash Commands [STANDARD]

### Built-in commands

| Command                      | What it does                                                         |
| ---------------------------- | -------------------------------------------------------------------- |
| `/clear`                     | Start a fresh conversation (frees up context)                        |
| `/compact [instructions]`    | Compress conversation history to save context space                  |
| `/cost`                      | See how many tokens you've used this session                         |
| `/rename <n>`                | Name your session so you can resume it later                         |
| `/resume`                    | Open a picker to resume a previous session                           |
| `/help`                      | List all available commands                                          |
| `/status`                    | Show account and subscription status                                 |
| `/model`                     | Switch the AI model                                                  |
| `/fast`                      | Toggle fast mode (faster output, same model)                         |
| `/vim`                       | Enable vim-style keybindings in the editor                           |
| `/terminal-setup`            | Configure terminal integration (e.g. for iTerm2)                     |
| `/doctor`                    | Check Claude Code environment health                                 |
| `/bug`                       | Report a bug to Anthropic                                            |
| `/init`                      | Generate a CLAUDE.md for the current project                         |
| `/review`                    | Review a pull request                                                |

### Utility skills

| Command                      | What it does                                                         |
| ---------------------------- | -------------------------------------------------------------------- |
| `/update-config`             | Configure Claude Code settings (hooks, permissions, env vars)        |
| `/keybindings-help`          | Customize keyboard shortcuts in `~/.claude/keybindings.json`         |
| `/loop [interval] <command>` | Run a command on a recurring interval                                |

### Global skills (available in every project)

Located in `~/.claude/skills/`. These work regardless of which project you're in.

| Skill | What it does |
|---|---|
| `/organize-project` | Scan files for naming convention violations, rename them, and update all wikilinks |

### Project-specific commands

Located in `<project>/.claude/commands/`. Only available inside that project. Type `/project_` to see them autocomplete.

**Naming convention**: The filename becomes the command, and must start with `project_`. A file named `project_quiz.md` is invoked as `/project_quiz`. A file named `project_study-note.md` becomes `/project_study-note`. Always use the `project_` prefix, lowercase, hyphens for multi-word names, no spaces.

Build these when you catch yourself giving Claude the same instructions repeatedly. See the Blueprint file for the slash command template.

---

## Keyboard Shortcuts [STANDARD]

### Input
| Shortcut | Action |
|---|---|
| `Enter` | Submit your message |
| `Option+Enter` | New line (for multiline input) |
| `Shift+Enter` | New line (iTerm2 / Ghostty — run `/terminal-setup` if it doesn't work) |
| `\ ` + `Enter` | New line (works in any terminal) |
| `Ctrl+C` | Cancel current generation |
| `Ctrl+D` | Exit Claude Code |
| `Ctrl+L` | Clear the screen (keeps history) |

### Navigation
| Shortcut | Action |
|---|---|
| `↑` / `↓` | Browse command history |
| `Shift+Tab` | Cycle permission mode: Normal → Auto-Accept → Plan |
| `Esc Esc` | Rewind conversation to a previous point |
| `Ctrl+T` | Toggle task list |
| `Ctrl+O` | Toggle verbose mode (shows tool details) |

### Tools
| Shortcut | Action |
|---|---|
| `Cmd+V` / `Ctrl+V` | Paste image from clipboard |
| `Option+T` | Toggle extended thinking (see section below) |
| `@filename` | Attach a file to your message |
| `?` | Show help menu |

---

## Permission Modes [STANDARD]

| Mode | How to activate | What it means |
|---|---|---|
| **Normal** | Default | Claude asks before every file write |
| **Auto-Accept** | `Shift+Tab` once | Claude writes files without asking — faster |
| **Plan** | `Shift+Tab` twice | Read-only, no changes — Claude just reads and reasons |

**When to use which:**
- **Auto-Accept**: When Claude is creating notes, journal entries, or other files you trust
- **Plan**: When you want to discuss, reason through a problem, or read PDFs without Claude modifying anything
- **Normal**: When you want to review each file write before it happens

---

## Referencing Files [STANDARD]

Use `@` to attach a file to your message:
```
Summarize the key points from @article.pdf
Create a study note based on @learning-objectives.pdf
```

You can also drag-and-drop files into the terminal window.

---

## Images [STANDARD]

Claude can analyze images — diagrams, charts, textbook screenshots, whiteboard photos, flowcharts, ECG tracings.

**How to use:**
- **Paste from clipboard**: `Cmd+V` (e.g., screenshot an image, then paste it)
- **Drag-and-drop**: Drag an image file into the terminal window
- **Reference by path**: `@diagram.png` in your message

Works mid-conversation with multiple images.

---

## Effort Level [STANDARD]

Controls how deeply Claude reasons. Higher effort = deeper thinking, slower responses, more tokens.

| Command | Effect | Persists? |
|---|---|---|
| `/effort auto` | Claude decides (default) | Yes |
| `/effort high` | Deeper reasoning | Yes |
| `/effort max` | Maximum depth (Opus 4.6 only) | Current session only |

For complex reasoning, `high` or `max` produces better results. For quick tasks, `auto` is fine.

---

## Interrupting Mid-Response [STANDARD]

If Claude is generating an answer and you see it going in the wrong direction, just start typing. Claude stops and adjusts — you don't have to wait for it to finish or press `Ctrl+C` first.

---

## Context Management [STANDARD]

Claude Code has a limited context window. As your conversation grows, older messages get compressed or dropped. Performance degrades before you hit the limit — not at the limit.

1. **Use `/compact` proactively** — after every major topic change within a session, not when Claude starts forgetting things.
2. **Name and resume sessions by topic** — separate sessions for separate topics. Each session has full context for its subject.
3. **Put reference information in CLAUDE.md, not in conversation** — anything Claude needs every session belongs in CLAUDE.md. It's loaded fresh every time. Conversation context is temporary; CLAUDE.md is permanent.
4. **Use memory for cross-session knowledge** — preferences and facts that should persist across all sessions and projects.
5. **Watch for degradation signals** — Claude contradicts itself, re-reads files it already read, gives generic responses, or ignores earlier instructions. When you notice this: `/compact` or start a fresh session.

---

## Extended Thinking [STANDARD]

**What it is**: When toggled on, Claude reasons through the problem step by step before responding. This is based on chain-of-thought reasoning — research shows this improves performance by up to 18% on complex tasks by letting the model work through intermediate steps rather than jumping to an answer.

**How to use it**: Press `Option+T` to toggle it on, then send your message. The toggle stays on until you press `Option+T` again.

**Important**: You must toggle it **before** sending your message. If you've already sent a prompt without thinking enabled, you can't add it retroactively. Instead: press `Ctrl+C` to cancel the generation, press `Option+T` to enable thinking, then resend your message.

**When to use it:**
- Complex multi-step reasoning (clinical cases, differential diagnosis, planning)
- Verifying whether a claim or mechanism is correct
- Generating high-quality questions that test reasoning, not recall
- Any task where you'd want a human expert to "think about it carefully first"

**When NOT to use it:**
- Simple requests (file creation, formatting, quick factual lookups)
- Straightforward tasks where speed matters more than depth

---

## Structuring Your Requests [STANDARD]

How you phrase your messages matters. These techniques help Claude give you better answers.

### Break complex requests into steps

Instead of one massive message, break it into a chain. Each message gets Claude's full attention:

```
Message 1: "Read @chapter-5.pdf and tell me the three main claims."
Message 2: "For claim 2, explain the supporting evidence."
Message 3: "Now create a study note on claim 2."
```

### Use XML tags for complex messages

When a single message needs background context, a task, and formatting requirements, separate them with tags:

```
<context>
I'm preparing for my exam next week.
I've already covered gas exchange and airway anatomy.
</context>

<task>
Explain ventilation-perfusion mismatch, building on what I already know.
</task>

<format>
Use 2-3 short paragraphs. Bold key terms.
End with a question to test my understanding.
</format>
```

### Ask Claude to show its reasoning

For complex questions:

```
Think through this step by step in <thinking> tags.
Then give me your conclusion in <answer> tags.
```

### Specify the output format

"Summarize this" is vague. "Summarize this in 3 bullet points, each one sentence, focusing on the clinical implications" is specific.

---

## Agents [STANDARD]

Claude Code can spawn sub-agents — independent workers that run tasks while you continue working.

**How to trigger**: Just ask in natural language:
- "In the background, summarize the PDF while I keep working here."
- "Research the latest guidelines on X while I work on Y."

| Type           | When to use                        | Behavior                                          |
| -------------- | ---------------------------------- | ------------------------------------------------- |
| **Foreground** | You need the result before continuing | Claude pauses, runs the agent, returns the result |
| **Background** | The task is independent of your work  | Agent runs while you keep working; notified on completion |

**When agents are NOT the right tool:**
- Tasks that need your decisions mid-way
- Simple, quick tasks — just ask directly

---

## Web Search [STANDARD]

Claude Code can search the web and fetch pages. Useful when:
- Materials are incomplete or unclear
- You want to check whether a guideline, technique, or fact is current
- You want to verify a claim

Just ask: "Search for the latest guidelines on..." or "Look up whether..."

---

## Memory System [STANDARD]

Claude remembers things across sessions via files in:
```
~/.claude/projects/.../memory/MEMORY.md
```

This is how Claude knows who you are, what you're working on, and your preferences. If Claude learns something that should persist, ask it to save it to memory.

---

## Essential Commands Cheat Sheet [CUSTOMIZE]

[Keep the standard rows. Add project-specific commands at the top.]

| Action                | Command                                                            |
| --------------------- | ------------------------------------------------------------------ |
| Start working         | Just start — Claude reads CLAUDE.md automatically                  |
| Run a project command | `/project_<command-name>` (type `/project_` to see autocomplete)   |
| Organize files        | `/organize-project`                                                |
| Save context          | `/compact`                                                         |
| Switch topic          | `/rename <session-name>` then `/clear` then start fresh            |
| Resume later          | `/resume`                                                          |
| Attach a file         | `@filename.pdf` in your message                                    |
| Background task       | "In the background, do X while I work on Y"                        |
| Deep reasoning        | Press `Option+T` before sending                                    |
| Structure a request   | Use XML tags: `<context>`, `<task>`, `<format>`                    |
| Break down a task     | Send each step as a separate message                               |

---

## Sessions Log [CUSTOMIZE]

Track your named sessions here so you can quickly find and resume the right one. Update this after each session.

| Session name | Purpose | Status |
|---|---|---|
| `config` | Project setup and file management | active |

Use `/rename <n>` to name a session, `/resume` to pick from the list.

---

## When Things Go Wrong [STANDARD]

### Claude starts giving generic or shallow responses
**What's happening**: Context window is filling up.
**Fix**: Run `/compact` or start a fresh session with `/clear`. If it keeps happening, your CLAUDE.md might have too many instructions — review and trim.

### Claude ignores instructions it followed earlier
**What's happening**: Context degradation.
**Fix**: `/compact` with specific instructions about what to preserve, or start a fresh session. If the ignored instruction is in CLAUDE.md and it's a new session, the CLAUDE.md might be too long — move content to `.claude/rules/`.

### Claude contradicts itself or re-reads files it already read
**What's happening**: Compaction has lost important context, or the session is too long.
**Fix**: Start a fresh session. If the lost context is critical, add it to CLAUDE.md or a reference file.

### Claude keeps making the same mistake across sessions
**What's happening**: The mistake isn't captured anywhere persistent.
**Fix**: Add it to the Corrections Log in CLAUDE.md.

### Claude does too much or adds things you didn't ask for
**What's happening**: The instructions are too vague, or Claude is inferring intent.
**Fix**: Be more explicit. Claude 4 models follow precise instructions well.

### You can't remember which session had your work
**Fix**: Keep the Sessions Log above updated. Use descriptive names with `/rename`.

---

## CLAUDE.md Maintenance [STANDARD]

| When | Do this |
|---|---|
| Starting a project | Write the initial CLAUDE.md using the Blueprint |
| After 2-3 sessions | Review: remove what's unused, add what you keep repeating |
| When Claude repeats a mistake | Add it to the Corrections Log in CLAUDE.md |
| When CLAUDE.md exceeds ~150 lines | Move specialized content to `.claude/rules/` |
| When Claude ignores instructions | Too many instructions — cut, consolidate, or modularize |
| When preferences change | Update the How to Interact section |
| End of project | Archive — the CLAUDE.md is a record of how you worked |
````

---

# Phase 2: Add Tools When You Feel Friction

**When to build a slash command**: When you catch yourself giving Claude the same instructions for the third time. Not before.

**When to add a rule file**: When you have instructions that apply only in certain contexts (e.g., only during journaling, only when working with PDFs, only when organizing files).

### Slash Command Template

Save in `.claude/commands/` as `project_<command-name>.md`. The filename becomes the command — `project_quiz.md` is invoked as `/project_quiz`, `project_study-note.md` becomes `/project_study-note`. Always use `project_` prefix, lowercase, hyphens for multi-word names, no spaces.

```markdown
[One-line description]: $ARGUMENTS

---

## Behavior
[What Claude should do. Use positive instructions — state what to do, and if
boundaries are needed, pair them with alternatives and reasoning.]

## Output Example
[Show what the output SHOULD look like, then what it should NOT look like.]

<example type="positive">
[A realistic, complete example of desired output]
</example>

<example type="negative">
[An example of wrong output]
Reason: [Why this is wrong — helps Claude generalize the boundary]
</example>

## Edge Cases
[What should Claude do when the input is ambiguous, missing, or unusual?]
- If no topic is specified: ask the user what topic they want
- If the topic is too broad: suggest 2-3 narrower subtopics to choose from
- If source material is missing: note the gap and work with what's available

## Verification
[How Claude checks its own output before presenting it.]

## File Persistence
[Where to save, how to name the file, frontmatter template.]
[Include duplicate checking if relevant.]

## At the End
[What to say/offer after completing the command.]
```

### Rule File Template

Save in `.claude/rules/` as a `.md` file:

```markdown
---
globs: ["**/journal-*.md", "**/journal/**"]
---
# Journal Entry Rules
- Ask one reflection question at a time, wait for the answer before asking the next
- End every entry with a "pattern I noticed" observation from this and recent entries
- Save entries as `journal-YYYY-MM-DD.md`
```

The glob pattern tells Claude Code to load these rules only when working with matching files.

---

# Phase 3: Settle Into a Rhythm

Once your CLAUDE.md is tuned and you have 1-2 slash commands, your sessions should follow a predictable loop. The exact loop depends on the project type, but the principle is the same: **engage actively, don't just consume output.**

The general rhythm:

```
1. INPUT     — Bring something to the session (a question, a file, an event, a thought)
2. PROCESS   — Work through it with Claude (discuss, organize, draft, plan)
3. OUTPUT    — Produce something concrete (a note, a decision, a plan, an entry)
4. REVIEW    — Revisit and refine (check work, track progress, adjust approach)
```

### The first few sessions

- After your first real session, you'll notice something Claude does that you didn't anticipate — add it to Corrections Log or adjust How to Interact
- After 2-3 sessions, you'll have a feel for which recurring tasks need slash commands — build them then, not before
- After a week, review CLAUDE.md: cut what you never use, add what you keep repeating

### Context management during sessions

Context is the single most important resource to manage. Performance doesn't degrade at the context limit — it degrades well before, in the upper portion of the window.

1. **Use `/compact` proactively** — after every major topic change, not when Claude starts forgetting. Your Compaction Instructions section tells `/compact` what to preserve.
2. **One session, one purpose** — separate sessions for separate topics. Name them with `/rename`.
3. **CLAUDE.md is permanent, conversation is temporary** — anything Claude needs every session belongs in CLAUDE.md.
4. **Use memory for cross-project knowledge** — preferences and facts that should persist across all projects.
5. **Watch for degradation signals** — contradicts itself, re-reads files, gives generic responses, ignores instructions. `/compact` or start fresh.

---

# Project Archetypes

Starting points, not rigid templates. Mix and match.

## Study / Learning
**Role**: Tutor and study partner specializing in [domain]
**Core loop**: Read material → Discuss and build notes → Quiz/case → Review gaps
**Typical commands**: `/project_study-note`, `/project_quiz`, `/project_case`, `/project_explain`
**Interaction style**: Socratic — challenge reasoning, force active recall
**Verification**: Check notes against learning objectives; check quiz questions test reasoning not recall
**Example exchange**: User asks a question → Claude asks what user already knows → builds from there

## Journaling / Personal Development
**Role**: Reflective coach and accountability partner
**Core loop**: Check in → Reflect on prompt → Write entry → Track patterns
**Typical commands**: `/project_journal`, `/project_weekly-review`, `/project_check-in`
**Interaction style**: Coaching — ask good questions, notice patterns, hold accountable
**Verification**: Confirm reflection questions are open-ended; check patterns reference previous entries
**Example exchange**: User says "let's journal" → Claude asks one focused question → deepens with follow-ups

## Productivity / Organization
**Role**: Productivity assistant and system operator
**Core loop**: Review inbox/calendar → Triage and prioritize → Execute → Report status
**Typical commands**: `/project_daily-review`, `/project_inbox-zero`, `/project_weekly-plan`
**Interaction style**: Direct and systematic — clear actions, minimal discussion
**Connected tools**: Google Calendar, Gmail, task managers
**Verification**: Confirm no events are double-booked; confirm emails are categorized before acting
**Example exchange**: User starts session → Claude pulls today's calendar and inbox → presents prioritized action list

## Writing / Creative
**Role**: Writing collaborator and editor
**Core loop**: Outline → Draft → Get feedback → Revise
**Typical commands**: `/project_draft`, `/project_feedback`, `/project_outline`
**Interaction style**: Collaborative — build on ideas, offer alternatives, challenge weak arguments
**Verification**: Check draft hits target audience, stays within word count, follows style guide
**Example exchange**: User shares a rough idea → Claude helps shape the structure → user writes, Claude edits

## Research / Knowledge Management
**Role**: Research assistant and knowledge organizer
**Core loop**: Gather sources → Summarize and link → Identify gaps → Deep-dive
**Typical commands**: `/project_summarize`, `/project_literature-note`, `/project_connect`
**Interaction style**: Thorough — verify claims, cross-reference, flag contradictions
**Verification**: Confirm all claims are sourced; confirm wikilinks connect to existing notes
**Example exchange**: User drops a PDF → Claude summarizes → user asks about a specific section → deep-dive

## Health / Fitness / Habits
**Role**: Tracking assistant and coach
**Core loop**: Log activity → Review trends → Adjust plan → Reflect
**Typical commands**: `/project_log`, `/project_weekly-stats`, `/project_adjust-plan`
**Interaction style**: Encouraging but honest — celebrate progress, flag stalls
**Verification**: Check logged data matches tracking format; flag impossible values
**Example exchange**: User logs workout → Claude notes trend vs. plan → suggests adjustment if needed

---

# Reference: Making Claude More Effective — Evidence-Based Techniques

These techniques are ranked by impact. The top three are confirmed by peer-reviewed research, all major AI provider documentation, and extensive practitioner testing.

**1. Be specific and explicit.** All three major AI providers converge on this as the single most impactful thing you can do. "Keep it to 2-3 sentences" beats "be concise."

**2. Use examples (positive and negative).** 2-5 examples dramatically improve performance (Brown et al. 2020, NeurIPS). Positive examples show what good looks like. Negative examples define boundaries.

**3. Let Claude think through complex tasks.** Chain-of-thought reasoning improved performance by up to 18% on complex tasks (Wei et al. 2022, NeurIPS). Press `Option+T` or use `<thinking>`/`<answer>` tags manually.

**4. Break complex requests into steps.** Each step gets Claude's full attention (Zhou et al. 2022, ICLR 2023).

**5. Give Claude a specific, detailed role.** The more specific — domain, approach, personality — the more focused the behavior.

**6. Explain the reasoning behind rules.** Claude generalizes from reasoning and applies the principle beyond the literal rule.

**7. Tell Claude how to verify its work.** Self-check criteria are the highest-leverage addition to any prompt.

**8. Structure complex requests with XML tags.** Claude was specifically trained with XML tags. Use `<context>`, `<task>`, `<format>` to prevent confusion.

**9. Match the tone you want.** Claude mirrors the style of its prompt. Write your CLAUDE.md in the tone you want.

---

# Reference: Engaging Actively (Not Just Generating Output)

### What works
1. **Forcing active retrieval** — quizzing and cases force recall, not re-reading
2. **Immediate, mechanism-based feedback** — Claude explains the mechanism, not just the fact
3. **Adapting to your level** — CLAUDE.md tells Claude who you are
4. **Challenging your reasoning** — Socratic/coaching setup exposes gaps
5. **Connecting concepts** — wikilinks build the knowledge web experts have

### What doesn't work
1. **It can't replace doing the work** — a perfect output you never engage with taught you nothing
2. **It can make you feel productive without progress** — generating files without engaging is just creating files
3. **It can be wrong** — always verify surprising claims

### The active loop (applies to any project type)

```
Don't do this:
  Input → /command → file saved → feel productive → move on

Do this:
  Input → engage with Claude first (discuss, get challenged, ask questions)
       → /command (now the output captures YOUR thinking, not just generated content)
       → review the output critically → update with what you missed
       → come back later → re-engage
```

---

# CLAUDE.md Maintenance

| When | Do this |
|---|---|
| Starting a project | Write the initial CLAUDE.md (keep it lean) |
| After 2-3 sessions | Review: remove unused, add repeated |
| When Claude repeats a mistake | Add to Corrections Log |
| When CLAUDE.md exceeds ~150 lines | Move content to `.claude/rules/` |
| When Claude ignores instructions | Too many instructions — cut or modularize |
| When preferences change | Update How to Interact |
| End of project | Archive |

---

# Checklist: Verify Setup Is Complete

- [ ] Project folder created in Obsidian vault
- [ ] `.claude/commands/` directory exists
- [ ] `.claude/rules/` directory exists
- [ ] CLAUDE.md saved — under 150 lines, all sections filled in
- [ ] Example Exchange populated with at least one positive and one negative example
- [ ] Instructions.md generated and customized for this project
- [ ] Existing materials scanned and organized (if any)
- [ ] First session named with `/rename`
- [ ] Session added to the Sessions Log in Instructions.md
