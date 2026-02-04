use base64::Engine;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::Serialize;

// ---------------------------------------------------------------------------
// 11. VOICE_LIST
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize)]
pub struct VoiceInfo {
    pub name: String,
    pub locale: String,
    pub gender: String,
}

pub static VOICE_LIST: Lazy<Vec<VoiceInfo>> = Lazy::new(|| {
    vec![
        VoiceInfo { name: "en-US-AriaNeural".into(),   locale: "en-US".into(), gender: "female".into() },
        VoiceInfo { name: "en-US-GuyNeural".into(),    locale: "en-US".into(), gender: "male".into()   },
        VoiceInfo { name: "en-GB-SoniaNeural".into(),  locale: "en-GB".into(), gender: "female".into() },
        VoiceInfo { name: "en-GB-RyanNeural".into(),   locale: "en-GB".into(), gender: "male".into()   },
        VoiceInfo { name: "fr-FR-DeniseNeural".into(), locale: "fr-FR".into(), gender: "female".into() },
        VoiceInfo { name: "de-DE-KatjaNeural".into(),  locale: "de-DE".into(), gender: "female".into() },
        VoiceInfo { name: "es-ES-HelenaNeural".into(), locale: "es-ES".into(), gender: "female".into() },
        VoiceInfo { name: "ja-JP-AzukiNeural".into(),  locale: "ja-JP".into(), gender: "female".into() },
    ]
});

// ---------------------------------------------------------------------------
// 12. random_delay
// ---------------------------------------------------------------------------

pub fn random_delay() -> tokio::time::Duration {
    let ms = rand::thread_rng().gen_range(200..=800);
    tokio::time::Duration::from_millis(ms)
}

// ---------------------------------------------------------------------------
// 1. mock_chat_response  (keyword-based, ~20 canned responses)
// ---------------------------------------------------------------------------

pub fn mock_chat_response(message: &str) -> String {
    let low = message.to_lowercase();

    if low.contains("hello") || low.contains("hi ") || low.contains("hey") || low.starts_with("hi") {
        return "Hello! Welcome to Nexus AI. I'm here to help you with anything you need \
today. Whether you have questions about technology, need help drafting a document, \
or just want to explore ideas, feel free to ask.\n\n\
I can assist with coding, writing, math, translations, and much more. \
What would you like to work on?".into();
    }

    if low.contains("how are you") || low.contains("how do you do") {
        return "I'm running at full capacity and ready to assist! As an AI, I don't \
experience feelings the way humans do, but I'm optimized and eager to help you \
tackle whatever challenge you bring my way.\n\n\
Think of me as a highly capable digital colleague who never needs coffee breaks. \
What can I help you with today?".into();
    }

    if low.contains("code") || low.contains("programming") || low.contains("debug") || low.contains("function") {
        return "I'd be happy to help with your coding question! Writing clean, efficient \
code is one of my core strengths. Here are a few things I can do for you:\n\n\
- Write functions, classes, or entire modules in most popular languages\n\
- Debug existing code and explain what went wrong\n\
- Suggest performance optimizations and best practices\n\
- Explain algorithms and data structures with examples\n\n\
Could you share the specific code or describe the problem you're working on? \
The more context you provide, the more targeted my help will be.".into();
    }

    if low.contains("write") || low.contains("essay") || low.contains("article") || low.contains("blog") {
        return "I can definitely help you with writing! Whether it's a blog post, an essay, \
a report, or creative fiction, I'll tailor the style and tone to your needs.\n\n\
To get started, it would help to know:\n\
1. **Topic or subject** - What should the piece be about?\n\
2. **Audience** - Who will be reading this?\n\
3. **Tone** - Formal, conversational, persuasive, or something else?\n\
4. **Length** - A rough word count or number of paragraphs.\n\n\
Once I have those details, I'll draft something polished and ready to go. \
Just let me know!".into();
    }

    if low.contains("math") || low.contains("calcul") || low.contains("equation") || low.contains("algebra") {
        return "Mathematics is one of my strong suits! I can work through problems step by \
step so you not only get the answer but understand the reasoning behind it.\n\n\
I'm comfortable with:\n\
- Arithmetic, algebra, and geometry\n\
- Calculus (derivatives, integrals, series)\n\
- Linear algebra and matrix operations\n\
- Statistics and probability\n\
- Discrete math and number theory\n\n\
Go ahead and share the problem or concept you're working on, and I'll walk you \
through it clearly.".into();
    }

    if low.contains("translat") || low.contains("spanish") || low.contains("french") || low.contains("german") || low.contains("language") {
        return "I can help with translations across many languages! I support English, \
Spanish, French, German, Italian, Portuguese, Chinese, Japanese, Korean, Arabic, \
Russian, Hindi, and more.\n\n\
For the best results, please provide:\n\
- The text you'd like translated\n\
- The source language (or I can auto-detect it)\n\
- The target language\n\n\
I'll also preserve the tone and context of the original as closely as possible, \
rather than doing a purely literal word-for-word conversion. Paste your text \
whenever you're ready!".into();
    }

    if low.contains("history") || low.contains("science") || low.contains("explain") || low.contains("what is") || low.contains("who is") {
        return "Great question! I have a broad knowledge base covering history, science, \
philosophy, geography, culture, and many other domains.\n\n\
I'll do my best to give you a thorough yet accessible explanation. If the topic \
is particularly deep, I'll break it into digestible sections with key takeaways.\n\n\
Keep in mind that my training data has a knowledge cutoff, so for very recent \
events you may want to cross-reference with a live news source. That said, for \
most established topics I can provide detailed, well-structured answers. \
What would you like to know?".into();
    }

    if low.contains("story") || low.contains("creative") || low.contains("poem") || low.contains("fiction") {
        return "I love creative projects! Whether you need a short story, a poem, a character \
sketch, or an entire plot outline, I can bring ideas to life with vivid language \
and compelling narratives.\n\n\
To tailor the output, let me know:\n\
- **Genre** - Fantasy, sci-fi, romance, mystery, literary fiction, etc.\n\
- **Mood** - Dark and atmospheric, lighthearted, suspenseful, whimsical?\n\
- **Length** - Flash fiction, short story, or just an opening scene?\n\
- **Any specific elements** - Characters, settings, or themes you want included.\n\n\
I'll craft something original and engaging. Just say the word!".into();
    }

    if low.contains("business") || low.contains("startup") || low.contains("market") || low.contains("strategy") {
        return "I can provide insights on a wide range of business topics, from startup \
strategy to market analysis. Here's how I can help:\n\n\
- **Business Planning** - Structure your ideas into a coherent plan with \
  market sizing, competitive analysis, and financial projections.\n\
- **Marketing Strategy** - Identify target audiences, craft messaging, and \
  plan campaigns across digital and traditional channels.\n\
- **Operations** - Streamline workflows, evaluate tools, and improve efficiency.\n\
- **Growth** - Explore customer acquisition strategies, retention tactics, \
  and scaling approaches.\n\n\
Tell me about your business or idea, and I'll provide actionable, specific advice \
rather than generic platitudes.".into();
    }

    if low.contains("health") || low.contains("fitness") || low.contains("nutrition") || low.contains("diet") || low.contains("exercise") {
        return "I can share general information about health, fitness, and nutrition topics. \
Please note that I'm an AI assistant, not a medical professional, so always \
consult a qualified healthcare provider for personal medical advice.\n\n\
That said, I can help with:\n\
- General wellness information and evidence-based practices\n\
- Exercise programming concepts and workout structures\n\
- Nutritional guidelines and meal planning ideas\n\
- Understanding common health metrics and what they mean\n\
- Summarizing research on specific health topics\n\n\
What specific health or fitness topic would you like to explore?".into();
    }

    if low.contains("learn") || low.contains("study") || low.contains("education") || low.contains("course") || low.contains("teach") {
        return "Education is one of the most rewarding things I can help with! Whether \
you're a student preparing for exams or a lifelong learner exploring new fields, \
I can be your study partner.\n\n\
Here's what I offer:\n\
- **Concept explanations** - I'll break down complex ideas into simple terms \
  with analogies and examples.\n\
- **Practice problems** - I can generate exercises and walk you through solutions.\n\
- **Study plans** - Help you structure a learning path for any subject.\n\
- **Flashcard content** - Create Q&A pairs for memorization.\n\
- **Essay and paper feedback** - Review your writing for clarity and argument structure.\n\n\
What subject or skill are you looking to develop?".into();
    }

    if low.contains("weather") || low.contains("forecast") {
        return "I don't have access to real-time weather data, but I can help you understand \
weather concepts, climate patterns, and meteorological phenomena.\n\n\
For current forecasts, I'd recommend checking a service like the National Weather \
Service, Weather.com, or your device's built-in weather app. They pull live data \
from weather stations and satellites.\n\n\
However, if you have questions about climate science, seasonal patterns, severe \
weather preparedness, or how weather systems work, I'm happy to dive into those \
topics in detail!".into();
    }

    if low.contains("recipe") || low.contains("cook") || low.contains("food") || low.contains("meal") {
        return "I'd love to help with cooking! Whether you need a full recipe, substitution \
ideas, or meal planning advice, I've got you covered.\n\n\
I can provide:\n\
- Complete recipes with ingredient lists and step-by-step instructions\n\
- Cooking technique explanations for any skill level\n\
- Meal prep strategies for busy weeks\n\
- Dietary adaptation suggestions (vegetarian, gluten-free, low-carb, etc.)\n\
- Wine or beverage pairing recommendations\n\n\
What kind of dish or cuisine are you in the mood for? Or if you tell me what \
ingredients you have on hand, I can suggest recipes that use them.".into();
    }

    if low.contains("travel") || low.contains("trip") || low.contains("vacation") || low.contains("visit") {
        return "Travel planning is a great use of my capabilities! I can help you research \
destinations, plan itineraries, and prepare for your trip.\n\n\
Here's what I can assist with:\n\
- **Destination research** - Climate, culture, must-see attractions, hidden gems\n\
- **Itinerary building** - Day-by-day plans optimized for logistics and interests\n\
- **Budget planning** - Estimated costs for accommodation, food, transport\n\
- **Packing lists** - Tailored to your destination and season\n\
- **Cultural tips** - Local customs, etiquette, and useful phrases\n\n\
Where are you thinking of going, and what kind of experience are you looking for?".into();
    }

    if low.contains("music") || low.contains("song") || low.contains("album") || low.contains("artist") {
        return "Music is a fantastic topic! While I can't play audio, I can discuss music \
in great depth.\n\n\
I can help with:\n\
- Music theory concepts, chord progressions, and composition techniques\n\
- Artist biographies and discography overviews\n\
- Genre histories and evolution\n\
- Recommendations based on your taste\n\
- Songwriting tips and lyric feedback\n\n\
Whether you're a musician looking for theory help or a listener wanting to explore \
new sounds, let me know what you're interested in!".into();
    }

    if low.contains("movie") || low.contains("film") || low.contains("watch") || low.contains("show") || low.contains("series") {
        return "I enjoy discussing movies and TV shows! I can offer recommendations, \
analysis, and behind-the-scenes insights.\n\n\
Here's what I can help with:\n\
- **Recommendations** - Based on genres, themes, or titles you already enjoy\n\
- **Plot summaries** - Spoiler-free or detailed, your choice\n\
- **Analysis** - Themes, cinematography, character arcs, and storytelling techniques\n\
- **Comparisons** - How adaptations compare to source material\n\
- **Trivia** - Fun facts about production, casting, and cultural impact\n\n\
What are you in the mood for, or what would you like to discuss?".into();
    }

    if low.contains("joke") || low.contains("funny") || low.contains("humor") || low.contains("laugh") {
        return "Sure, here's one for you:\n\n\
Why do programmers prefer dark mode? Because light attracts bugs!\n\n\
And a bonus: A SQL query walks into a bar, sees two tables, and asks... \
\"Can I join you?\"\n\n\
I've got jokes across many categories - tech humor, wordplay, observational \
comedy, and clean one-liners. Want me to try a specific style, or shall I \
keep them coming?".into();
    }

    if low.contains("thank") || low.contains("thanks") || low.contains("appreciate") {
        return "You're very welcome! I'm glad I could help. If you have any more questions \
or need assistance with anything else, don't hesitate to ask.\n\n\
I'm here whenever you need me - whether it's five minutes from now or five months \
from now. Happy to be of service!".into();
    }

    if low.contains("bye") || low.contains("goodbye") || low.contains("see you") {
        return "Goodbye! It was great chatting with you. Remember, I'm available 24/7 \
whenever you need help with coding, writing, research, or anything else.\n\n\
Have a wonderful day, and don't hesitate to come back anytime!".into();
    }

    // Default / general fallback
    "That's an interesting question! Let me share my thoughts.\n\n\
Nexus AI is designed to be a versatile assistant that can help across a wide \
range of topics. While I may not have caught a specific keyword in your message, \
I'm still happy to help.\n\n\
Could you provide a bit more detail about what you're looking for? For example:\n\
- Are you looking for information or explanations on a topic?\n\
- Do you need help creating or editing something?\n\
- Would you like assistance solving a problem?\n\n\
The more context you give me, the more helpful and specific my response will be.".into()
}

// ---------------------------------------------------------------------------
// 2. mock_content  (platform + tone aware)
// ---------------------------------------------------------------------------

pub fn mock_content(platform: &str, tone: &str, prompt: &str) -> String {
    let topic = if prompt.len() > 60 { &prompt[..60] } else { prompt };

    match platform.to_lowercase().as_str() {
        "twitter" | "x" => match tone.to_lowercase().as_str() {
            "professional" => format!(
                "Excited to share our latest insights on {topic}. After extensive research \
and collaboration with industry leaders, we've identified key trends that are \
reshaping the landscape.\n\n\
Here's what every professional should know:\n\n\
1/ The market is shifting toward AI-driven solutions faster than predicted\n\
2/ Early adopters are seeing 3x productivity gains\n\
3/ Integration complexity is the #1 barrier to entry\n\n\
Full thread below with data and actionable takeaways.\n\n\
#Innovation #TechTrends #Leadership"
            ),
            "humorous" => format!(
                "Me: I'll just spend 5 minutes on {topic}\n\
Also me: *3 hours later* Did you know that...\n\n\
Seriously though, this rabbit hole is WORTH IT. Here's what I found that \
will either blow your mind or make you question everything you thought you knew.\n\n\
Thread incoming. Buckle up. No refunds on lost productivity.\n\n\
#Relatable #DeepDive #WorthIt"
            ),
            "inspirational" => format!(
                "Every great achievement starts with a single step. Today, I want to talk \
about {topic} and why it matters more than ever.\n\n\
The people who change the world aren't the ones who wait for perfect conditions. \
They're the ones who start with what they have and build from there.\n\n\
Your journey is unique. Your perspective matters. Keep pushing forward.\n\n\
#Motivation #Growth #NeverStopLearning"
            ),
            "formal" => format!(
                "We are pleased to announce our comprehensive analysis of {topic}. \
This examination draws upon peer-reviewed research and quantitative data \
from leading institutions.\n\n\
Key findings indicate significant developments in this area that warrant \
attention from stakeholders and decision-makers across sectors.\n\n\
A detailed report is available upon request.\n\n\
#Research #Analysis #Industry"
            ),
            _ => format!( // casual default
                "Just dove deep into {topic} and honestly? It's way more interesting than \
I expected.\n\n\
Here's the thing nobody talks about - the real magic happens when you look \
at it from a completely different angle. Changed my whole perspective.\n\n\
Anyone else been exploring this? Would love to hear your take!\n\n\
#Thoughts #Exploring #Community"
            ),
        },

        "linkedin" => match tone.to_lowercase().as_str() {
            "professional" | "formal" => format!(
                "I've been reflecting on {topic} and its implications for our industry.\n\n\
After 15 years in this space, I've noticed that the organizations achieving \
the most remarkable outcomes share three characteristics:\n\n\
1. They invest in their people before their technology\n\
2. They measure what matters, not what's easy to measure\n\
3. They embrace calculated risk-taking at every level\n\n\
The data supports this: companies that prioritize these pillars see 40% higher \
employee retention and 2.5x faster time-to-market on new initiatives.\n\n\
The question isn't whether to adapt - it's how quickly you can build the \
culture and infrastructure to support continuous evolution.\n\n\
I'd love to hear from leaders who have navigated similar transitions. \
What worked? What surprised you?\n\n\
#Leadership #Innovation #ProfessionalDevelopment #Strategy"
            ),
            "inspirational" => format!(
                "Three years ago, I was told that pursuing {topic} was a dead end. \
That the market was saturated. That I was too late.\n\n\
Today, I'm writing this post from a completely different position - one that \
exists because I didn't listen to the skeptics.\n\n\
Here's what I learned on that journey:\n\n\
The best time to start was yesterday. The second best time is now. But the \
real secret? It's not about timing at all. It's about persistence, adaptability, \
and surrounding yourself with people who challenge you to be better.\n\n\
To everyone in my network who is on the fence about taking that leap: the \
uncertainty you feel is not a warning sign. It's a growth signal.\n\n\
#CareerGrowth #Inspiration #TakeTheLeap"
            ),
            _ => format!(
                "Had an interesting conversation this week about {topic} that I wanted \
to share with my network.\n\n\
The consensus seems to be shifting. What was considered best practice \
even two years ago is now being questioned by some of the sharpest minds \
in the industry.\n\n\
My key takeaways:\n\
- Context matters more than frameworks\n\
- Speed of learning beats speed of execution\n\
- The people asking \"why\" are more valuable than those who only ask \"how\"\n\n\
Curious to hear different perspectives. Drop a comment with your experience!\n\n\
#Networking #Insights #OpenDiscussion"
            ),
        },

        "blog" => match tone.to_lowercase().as_str() {
            "professional" | "formal" => format!(
                "# A Comprehensive Look at {topic}\n\n\
## Introduction\n\n\
In recent years, the conversation around {topic} has evolved significantly. \
What was once a niche concern has become a central consideration for \
organizations of all sizes. This article examines the current state of \
affairs, the driving forces behind recent changes, and what practitioners \
should anticipate in the coming months.\n\n\
## The Current Landscape\n\n\
The data paints a compelling picture. According to industry benchmarks, \
adoption rates have increased by approximately 67% year-over-year, while \
implementation costs have decreased by nearly 40%. This convergence of \
accessibility and affordability is creating unprecedented opportunities.\n\n\
However, it would be an oversimplification to suggest that the path forward \
is without challenges. Integration complexity, skills gaps, and evolving \
regulatory requirements continue to present obstacles that require thoughtful \
navigation.\n\n\
## Key Considerations\n\n\
Organizations evaluating their approach to {topic} should focus on three \
critical areas:\n\n\
**1. Strategic Alignment** - Ensure that any initiative is directly tied to \
measurable business outcomes rather than pursued for its own sake.\n\n\
**2. Talent Development** - Invest in upskilling existing teams rather than \
relying solely on external hires. Internal champions drive sustainable adoption.\n\n\
**3. Iterative Implementation** - Start with contained pilot programs that \
can demonstrate value before scaling across the organization.\n\n\
## Conclusion\n\n\
The trajectory is clear: {topic} is not a passing trend but a fundamental \
shift in how value is created and delivered. The organizations that thrive \
will be those that approach it with both ambition and discipline."
            ),
            "humorous" => format!(
                "# The Absolutely Unhinged Guide to {topic}\n\n\
Look, I know what you're thinking. \"Not another article about {topic}.\" \
And honestly? Fair. But stick with me for five minutes, and I promise you'll \
either learn something useful or at least have a good laugh at my expense.\n\n\
## The Problem Nobody Wants to Admit\n\n\
Here's the dirty secret about {topic}: most people who claim to understand \
it are just really good at nodding confidently in meetings. I know this \
because I was one of those people for an embarrassingly long time.\n\n\
Then one Tuesday afternoon, someone actually asked me to explain it, and \
I had what scientists call an \"oh no\" moment.\n\n\
## What I Learned (The Hard Way)\n\n\
After falling down approximately 47 rabbit holes and consuming enough \
coffee to concern my doctor, here's what I actually figured out:\n\n\
**First**, it's simpler than everyone makes it sound. The jargon is the \
hardest part. Once you translate the buzzwords into plain English, the \
concepts are genuinely intuitive.\n\n\
**Second**, you don't need to understand everything to get started. \
That's like saying you need to understand internal combustion before \
you can drive a car.\n\n\
**Third** - and this is the important one - the people who get the best \
results aren't the smartest ones. They're the ones who just... start. \
Messy, imperfect, sometimes hilariously wrong. But they iterate faster \
than anyone else.\n\n\
## The Takeaway\n\n\
{topic} isn't magic. It's a tool. A really good tool, but still a tool. \
And like all tools, it works best when you stop overthinking and just \
pick it up."
            ),
            _ => format!(
                "# Understanding {topic}: What You Need to Know\n\n\
If you've been following developments in {topic}, you know things are \
moving fast. But cutting through the noise to find what actually matters \
can be a challenge. That's exactly what this post aims to do.\n\n\
## Why This Matters Now\n\n\
The timing isn't coincidental. Several converging trends have brought \
{topic} to the forefront: increasing demand for efficiency, evolving \
user expectations, and new capabilities that simply weren't available \
a few years ago.\n\n\
## The Practical Breakdown\n\n\
Let's skip the theory and focus on what you can actually do with this \
information:\n\n\
- **Start small** - Pick one area where {topic} can make an immediate \
  impact and prove value before expanding.\n\
- **Measure everything** - You can't improve what you don't measure. \
  Set clear baselines before making changes.\n\
- **Stay flexible** - The landscape is evolving quickly. What works \
  today might need adjustment in six months.\n\n\
## Wrapping Up\n\n\
{topic} represents a real opportunity for those willing to invest the \
time to understand it properly. The good news? You don't need to become \
an expert overnight. Consistent, incremental progress will get you \
further than any crash course."
            ),
        },

        "instagram" => match tone.to_lowercase().as_str() {
            "inspirational" => format!(
                "Some things are worth the wait. {topic} taught me that patience isn't \
passive - it's the most active form of trust.\n\n\
Every setback was a setup for something better. Every \"no\" redirected me \
toward a bigger \"yes.\"\n\n\
If you're in the messy middle right now, keep going. The view from the \
other side is worth every step.\n\n\
Save this for when you need the reminder.\n\n\
#Motivation #Growth #Mindset #KeepGoing #DailyInspiration #NeverGiveUp \
#PositiveVibes #Hustle #DreamBig #BelieveInYourself"
            ),
            "professional" => format!(
                "Behind the scenes of our latest project on {topic}.\n\n\
What looks effortless on the surface is the result of weeks of research, \
iteration, and collaboration. Every detail matters - from the initial \
concept to the final execution.\n\n\
Grateful for the team that makes it happen and the community that \
keeps us pushing forward.\n\n\
Swipe to see the process breakdown.\n\n\
#BehindTheScenes #Process #TeamWork #Creative #Design #Quality \
#Craftsmanship #Detail #ProfessionalLife"
            ),
            _ => format!(
                "Currently obsessed with {topic} and honestly, no regrets.\n\n\
There's something about diving into something new that just hits \
different. The learning curve is steep but the view from here? \
Totally worth it.\n\n\
Drop a comment if you've been exploring this too - would love to \
connect with fellow enthusiasts!\n\n\
#Exploring #NewPassion #LearningEveryDay #Community #Lifestyle \
#GoodVibes #Curious #Adventure"
            ),
        },

        "facebook" | _ => match tone.to_lowercase().as_str() {
            "professional" | "formal" => format!(
                "We're excited to share some important updates about {topic}.\n\n\
Over the past quarter, our team has been working diligently to analyze \
emerging trends and develop actionable strategies. The results have \
exceeded our expectations, and we want to share what we've learned \
with our community.\n\n\
Key highlights:\n\
- Engagement metrics are up 45% compared to the previous period\n\
- Our community has grown to over 10,000 active members\n\
- Three new partnership opportunities have been established\n\n\
We believe that transparency and knowledge-sharing strengthen our \
entire ecosystem. Stay tuned for a detailed report next week.\n\n\
Thank you for being part of this journey. Your feedback and support \
make all of this possible."
            ),
            "humorous" => format!(
                "Okay, real talk about {topic}.\n\n\
I spent way too long this weekend going down this rabbit hole and now \
I'm that person at dinner parties who won't stop talking about it. \
You know the type. I've become the type.\n\n\
But here's the thing - it's actually fascinating once you get past \
the initial \"why would anyone care about this\" phase. And now I'm \
on a mission to convert everyone I know.\n\n\
Consider this your formal warning. Next time we hang out, you WILL \
hear about {topic}. Resistance is futile.\n\n\
Who's already in this club? Let me know I'm not alone here."
            ),
            _ => format!(
                "Been thinking a lot about {topic} lately and wanted to share some \
thoughts with everyone.\n\n\
It's one of those things that seems straightforward on the surface, \
but the more you dig in, the more layers you discover. I've been \
researching it for a few weeks now and every day brings a new \
perspective.\n\n\
What I find most interesting is how it connects to so many other \
areas of our lives. The ripple effects are genuinely surprising.\n\n\
Has anyone else been looking into this? I'd love to start a \
conversation about it. Share your thoughts in the comments - \
all perspectives welcome!\n\n\
Also sharing a few resources in the comments for anyone who wants \
to learn more."
            ),
        },
    }
}

// ---------------------------------------------------------------------------
// 3. mock_code  (language-specific snippets)
// ---------------------------------------------------------------------------

pub fn mock_code(language: &str, description: &str) -> String {
    let desc_low = description.to_lowercase();

    match language.to_lowercase().as_str() {
        "rust" => {
            if desc_low.contains("api") || desc_low.contains("http") || desc_low.contains("server") {
                "use axum::{routing::get, Router, Json, extract::Path};\n\
use serde::{Deserialize, Serialize};\n\
use std::sync::Arc;\n\
use tokio::sync::RwLock;\n\n\
#[derive(Debug, Clone, Serialize, Deserialize)]\n\
struct Item {\n    \
    id: u64,\n    \
    name: String,\n    \
    description: String,\n    \
    created_at: String,\n\
}\n\n\
type Db = Arc<RwLock<Vec<Item>>>;\n\n\
async fn list_items(db: axum::extract::State<Db>) -> Json<Vec<Item>> {\n    \
    let items = db.read().await;\n    \
    Json(items.clone())\n\
}\n\n\
async fn get_item(\n    \
    Path(id): Path<u64>,\n    \
    db: axum::extract::State<Db>,\n\
) -> Result<Json<Item>, (axum::http::StatusCode, String)> {\n    \
    let items = db.read().await;\n    \
    items\n        \
        .iter()\n        \
        .find(|i| i.id == id)\n        \
        .cloned()\n        \
        .map(Json)\n        \
        .ok_or_else(|| {\n            \
            (axum::http::StatusCode::NOT_FOUND, format!(\"Item {} not found\", id))\n        \
        })\n\
}\n\n\
#[tokio::main]\n\
async fn main() {\n    \
    let db: Db = Arc::new(RwLock::new(Vec::new()));\n\n    \
    let app = Router::new()\n        \
        .route(\"/items\", get(list_items))\n        \
        .route(\"/items/:id\", get(get_item))\n        \
        .with_state(db);\n\n    \
    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await.unwrap();\n    \
    println!(\"Server running on http://localhost:3000\");\n    \
    axum::serve(listener, app).await.unwrap();\n\
}".into()
            } else {
                "use std::collections::HashMap;\n\n\
/// A generic data processor that transforms and filters collections.\n\
pub struct DataProcessor<T> {\n    \
    data: Vec<T>,\n    \
    filters: Vec<Box<dyn Fn(&T) -> bool>>,\n\
}\n\n\
impl<T: Clone + std::fmt::Debug> DataProcessor<T> {\n    \
    pub fn new(data: Vec<T>) -> Self {\n        \
        Self {\n            \
            data,\n            \
            filters: Vec::new(),\n        \
        }\n    \
    }\n\n    \
    pub fn add_filter<F: Fn(&T) -> bool + 'static>(mut self, f: F) -> Self {\n        \
        self.filters.push(Box::new(f));\n        \
        self\n    \
    }\n\n    \
    pub fn process(&self) -> Vec<T> {\n        \
        self.data\n            \
            .iter()\n            \
            .filter(|item| self.filters.iter().all(|f| f(item)))\n            \
            .cloned()\n            \
            .collect()\n    \
    }\n\n    \
    pub fn count(&self) -> usize {\n        \
        self.process().len()\n    \
    }\n\
}\n\n\
fn main() {\n    \
    let numbers: Vec<i32> = (1..=100).collect();\n\n    \
    let result = DataProcessor::new(numbers)\n        \
        .add_filter(|n| *n % 2 == 0)       // even numbers\n        \
        .add_filter(|n| *n > 20)            // greater than 20\n        \
        .add_filter(|n| *n < 80)            // less than 80\n        \
        .process();\n\n    \
    println!(\"Filtered results: {:?}\", result);\n    \
    println!(\"Count: {}\", result.len());\n\
}".into()
            }
        }

        "python" => {
            if desc_low.contains("api") || desc_low.contains("http") || desc_low.contains("server") || desc_low.contains("flask") || desc_low.contains("fastapi") {
                "from fastapi import FastAPI, HTTPException\n\
from pydantic import BaseModel\n\
from typing import Optional\n\
from datetime import datetime\n\
import uuid\n\n\
app = FastAPI(title=\"Items API\", version=\"1.0.0\")\n\n\
class ItemCreate(BaseModel):\n    \
    name: str\n    \
    description: str\n    \
    price: float\n    \
    category: Optional[str] = None\n\n\
class Item(ItemCreate):\n    \
    id: str\n    \
    created_at: datetime\n    \
    updated_at: datetime\n\n\
# In-memory storage\n\
items_db: dict[str, Item] = {}\n\n\
@app.post(\"/items\", response_model=Item, status_code=201)\n\
async def create_item(item: ItemCreate):\n    \
    item_id = str(uuid.uuid4())\n    \
    now = datetime.utcnow()\n    \
    db_item = Item(\n        \
        id=item_id,\n        \
        created_at=now,\n        \
        updated_at=now,\n        \
        **item.model_dump()\n    \
    )\n    \
    items_db[item_id] = db_item\n    \
    return db_item\n\n\
@app.get(\"/items\")\n\
async def list_items(category: Optional[str] = None, limit: int = 50):\n    \
    results = list(items_db.values())\n    \
    if category:\n        \
        results = [i for i in results if i.category == category]\n    \
    return results[:limit]\n\n\
@app.get(\"/items/{item_id}\", response_model=Item)\n\
async def get_item(item_id: str):\n    \
    if item_id not in items_db:\n        \
        raise HTTPException(status_code=404, detail=\"Item not found\")\n    \
    return items_db[item_id]\n\n\
@app.delete(\"/items/{item_id}\", status_code=204)\n\
async def delete_item(item_id: str):\n    \
    if item_id not in items_db:\n        \
        raise HTTPException(status_code=404, detail=\"Item not found\")\n    \
    del items_db[item_id]".into()
            } else {
                "from dataclasses import dataclass, field\n\
from typing import Any, Callable, Optional\n\
from collections import defaultdict\n\
import json\n\
from pathlib import Path\n\n\
@dataclass\n\
class DataPipeline:\n    \
    \"\"\"A composable data processing pipeline.\"\"\"\n    \
    name: str\n    \
    steps: list[Callable] = field(default_factory=list)\n    \
    _results: list[Any] = field(default_factory=list, repr=False)\n\n    \
    def add_step(self, func: Callable, description: str = \"\") -> 'DataPipeline':\n        \
        \"\"\"Add a processing step to the pipeline.\"\"\"\n        \
        func.__doc__ = description or func.__doc__ or \"No description\"\n        \
        self.steps.append(func)\n        \
        return self\n\n    \
    def run(self, data: Any) -> Any:\n        \
        \"\"\"Execute all pipeline steps sequentially.\"\"\"\n        \
        result = data\n        \
        for i, step in enumerate(self.steps):\n            \
            try:\n                \
                result = step(result)\n                \
                self._results.append({\n                    \
                    'step': i,\n                    \
                    'name': step.__doc__,\n                    \
                    'status': 'success',\n                \
                })\n            \
            except Exception as e:\n                \
                self._results.append({\n                    \
                    'step': i,\n                    \
                    'name': step.__doc__,\n                    \
                    'status': 'error',\n                    \
                    'error': str(e),\n                \
                })\n                \
                raise\n        \
        return result\n\n    \
    def summary(self) -> str:\n        \
        \"\"\"Return a summary of the last run.\"\"\"\n        \
        lines = [f\"Pipeline: {self.name}\"]\n        \
        for r in self._results:\n            \
            status = 'OK' if r['status'] == 'success' else 'FAIL'\n            \
            lines.append(f\"  [{status}] Step {r['step']}: {r['name']}\")\n        \
        return '\\n'.join(lines)\n\n\n\
# Example usage\n\
if __name__ == \"__main__\":\n    \
    pipeline = (\n        \
        DataPipeline(\"text-processor\")\n        \
        .add_step(str.strip, \"Remove whitespace\")\n        \
        .add_step(str.lower, \"Lowercase text\")\n        \
        .add_step(lambda s: s.replace('  ', ' '), \"Normalize spaces\")\n        \
        .add_step(lambda s: s.split(), \"Tokenize\")\n        \
        .add_step(lambda words: [w for w in words if len(w) > 2], \"Filter short words\")\n    \
    )\n\n    \
    result = pipeline.run(\"  Hello World  This Is A  Test  \")\n    \
    print(f\"Result: {result}\")\n    \
    print(pipeline.summary())".into()
            }
        }

        "javascript" | "js" => {
            "class EventEmitter {\n  \
  #listeners = new Map();\n  \
  #maxListeners = 10;\n\n  \
  on(event, callback) {\n    \
    if (!this.#listeners.has(event)) {\n      \
      this.#listeners.set(event, []);\n    \
    }\n    \
    const listeners = this.#listeners.get(event);\n    \
    if (listeners.length >= this.#maxListeners) {\n      \
      console.warn(`MaxListenersExceeded: ${event} has ${listeners.length} listeners`);\n    \
    }\n    \
    listeners.push(callback);\n    \
    return () => this.off(event, callback);\n  \
  }\n\n  \
  off(event, callback) {\n    \
    const listeners = this.#listeners.get(event);\n    \
    if (listeners) {\n      \
      const idx = listeners.indexOf(callback);\n      \
      if (idx !== -1) listeners.splice(idx, 1);\n    \
    }\n  \
  }\n\n  \
  emit(event, ...args) {\n    \
    const listeners = this.#listeners.get(event) || [];\n    \
    for (const listener of [...listeners]) {\n      \
      try {\n        \
        listener(...args);\n      \
      } catch (error) {\n        \
        console.error(`Error in listener for ${event}:`, error);\n      \
      }\n    \
    }\n    \
    return listeners.length > 0;\n  \
  }\n\n  \
  once(event, callback) {\n    \
    const wrapper = (...args) => {\n      \
      this.off(event, wrapper);\n      \
      callback(...args);\n    \
    };\n    \
    return this.on(event, wrapper);\n  \
  }\n\
}\n\n\
// Usage example\n\
const bus = new EventEmitter();\n\n\
const unsubscribe = bus.on('user:login', (user) => {\n  \
  console.log(`Welcome back, ${user.name}!`);\n\
});\n\n\
bus.once('app:ready', () => {\n  \
  console.log('Application initialized successfully.');\n\
});\n\n\
bus.emit('user:login', { name: 'Alice', role: 'admin' });\n\
bus.emit('app:ready');\n\
unsubscribe();".into()
        }

        "typescript" | "ts" => {
            "interface Result<T, E = Error> {\n  \
  ok: boolean;\n  \
  value?: T;\n  \
  error?: E;\n\
}\n\n\
class AsyncQueue<T> {\n  \
  private queue: T[] = [];\n  \
  private processing = false;\n  \
  private concurrency: number;\n  \
  private handler: (item: T) => Promise<Result<void>>;\n  \
  private results: Result<void>[] = [];\n\n  \
  constructor(\n    \
    handler: (item: T) => Promise<Result<void>>,\n    \
    concurrency = 3\n  \
  ) {\n    \
    this.handler = handler;\n    \
    this.concurrency = concurrency;\n  \
  }\n\n  \
  enqueue(...items: T[]): void {\n    \
    this.queue.push(...items);\n    \
    this.process();\n  \
  }\n\n  \
  private async process(): Promise<void> {\n    \
    if (this.processing) return;\n    \
    this.processing = true;\n\n    \
    while (this.queue.length > 0) {\n      \
      const batch = this.queue.splice(0, this.concurrency);\n      \
      const promises = batch.map(async (item) => {\n        \
        try {\n          \
          await this.handler(item);\n          \
          return { ok: true } as Result<void>;\n        \
        } catch (error) {\n          \
          return { ok: false, error: error as Error } as Result<void>;\n        \
        }\n      \
      });\n\n      \
      const batchResults = await Promise.allSettled(promises);\n      \
      for (const result of batchResults) {\n        \
        if (result.status === 'fulfilled') {\n          \
          this.results.push(result.value);\n        \
        }\n      \
      }\n    \
    }\n\n    \
    this.processing = false;\n  \
  }\n\n  \
  getResults(): Result<void>[] {\n    \
    return [...this.results];\n  \
  }\n\n  \
  get pending(): number {\n    \
    return this.queue.length;\n  \
  }\n\
}\n\n\
// Usage\n\
const queue = new AsyncQueue<string>(async (url) => {\n  \
  const response = await fetch(url);\n  \
  if (!response.ok) throw new Error(`HTTP ${response.status}`);\n  \
  console.log(`Fetched: ${url} (${response.status})`);\n  \
  return { ok: true };\n\
}, 5);\n\n\
queue.enqueue(\n  \
  'https://api.example.com/users',\n  \
  'https://api.example.com/posts',\n  \
  'https://api.example.com/comments'\n\
);".into()
        }

        "go" | "golang" => {
            "package main\n\n\
import (\n\t\
\"context\"\n\t\
\"encoding/json\"\n\t\
\"fmt\"\n\t\
\"log\"\n\t\
\"net/http\"\n\t\
\"sync\"\n\t\
\"time\"\n\
)\n\n\
type Item struct {\n\t\
ID          string    `json:\"id\"`\n\t\
Name        string    `json:\"name\"`\n\t\
Description string    `json:\"description\"`\n\t\
CreatedAt   time.Time `json:\"created_at\"`\n\
}\n\n\
type Store struct {\n\t\
mu    sync.RWMutex\n\t\
items map[string]Item\n\
}\n\n\
func NewStore() *Store {\n\t\
return &Store{items: make(map[string]Item)}\n\
}\n\n\
func (s *Store) Get(id string) (Item, bool) {\n\t\
s.mu.RLock()\n\t\
defer s.mu.RUnlock()\n\t\
item, ok := s.items[id]\n\t\
return item, ok\n\
}\n\n\
func (s *Store) Set(item Item) {\n\t\
s.mu.Lock()\n\t\
defer s.mu.Unlock()\n\t\
s.items[item.ID] = item\n\
}\n\n\
func (s *Store) List() []Item {\n\t\
s.mu.RLock()\n\t\
defer s.mu.RUnlock()\n\t\
result := make([]Item, 0, len(s.items))\n\t\
for _, item := range s.items {\n\t\t\
result = append(result, item)\n\t\
}\n\t\
return result\n\
}\n\n\
func main() {\n\t\
store := NewStore()\n\t\
mux := http.NewServeMux()\n\n\t\
mux.HandleFunc(\"GET /items\", func(w http.ResponseWriter, r *http.Request) {\n\t\t\
w.Header().Set(\"Content-Type\", \"application/json\")\n\t\t\
json.NewEncoder(w).Encode(store.List())\n\t\
})\n\n\t\
mux.HandleFunc(\"GET /items/{id}\", func(w http.ResponseWriter, r *http.Request) {\n\t\t\
id := r.PathValue(\"id\")\n\t\t\
item, ok := store.Get(id)\n\t\t\
if !ok {\n\t\t\t\
http.Error(w, \"not found\", http.StatusNotFound)\n\t\t\t\
return\n\t\t\
}\n\t\t\
w.Header().Set(\"Content-Type\", \"application/json\")\n\t\t\
json.NewEncoder(w).Encode(item)\n\t\
})\n\n\t\
srv := &http.Server{Addr: \":8080\", Handler: mux}\n\t\
log.Printf(\"Listening on :8080\")\n\t\
log.Fatal(srv.ListenAndServe())\n\
}".into()
        }

        "java" => {
            "import java.util.*;\n\
import java.util.concurrent.*;\n\
import java.util.stream.*;\n\n\
public class TaskScheduler<T> {\n    \
    private final BlockingQueue<Task<T>> taskQueue;\n    \
    private final ExecutorService executor;\n    \
    private final Map<String, TaskResult<T>> results;\n    \
    private volatile boolean running;\n\n    \
    public record Task<T>(String id, Callable<T> callable, int priority) \n        \
        implements Comparable<Task<T>> {\n        \
        @Override\n        \
        public int compareTo(Task<T> other) {\n            \
            return Integer.compare(other.priority, this.priority);\n        \
        }\n    \
    }\n\n    \
    public record TaskResult<T>(String taskId, T value, Exception error, long durationMs) {\n        \
        public boolean isSuccess() { return error == null; }\n    \
    }\n\n    \
    public TaskScheduler(int workerCount) {\n        \
        this.taskQueue = new PriorityBlockingQueue<>();\n        \
        this.executor = Executors.newFixedThreadPool(workerCount);\n        \
        this.results = new ConcurrentHashMap<>();\n        \
        this.running = true;\n    \
    }\n\n    \
    public String submit(Callable<T> callable, int priority) {\n        \
        String id = UUID.randomUUID().toString();\n        \
        taskQueue.offer(new Task<>(id, callable, priority));\n        \
        processNext();\n        \
        return id;\n    \
    }\n\n    \
    private void processNext() {\n        \
        executor.submit(() -> {\n            \
            Task<T> task = taskQueue.poll();\n            \
            if (task == null) return;\n            \
            long start = System.currentTimeMillis();\n            \
            try {\n                \
                T result = task.callable().call();\n                \
                long duration = System.currentTimeMillis() - start;\n                \
                results.put(task.id(), new TaskResult<>(task.id(), result, null, duration));\n            \
            } catch (Exception e) {\n                \
                long duration = System.currentTimeMillis() - start;\n                \
                results.put(task.id(), new TaskResult<>(task.id(), null, e, duration));\n            \
            }\n        \
        });\n    \
    }\n\n    \
    public Optional<TaskResult<T>> getResult(String taskId) {\n        \
        return Optional.ofNullable(results.get(taskId));\n    \
    }\n\n    \
    public void shutdown() {\n        \
        running = false;\n        \
        executor.shutdown();\n    \
    }\n\
}".into()
        }

        "c" => {
            "#include <stdio.h>\n\
#include <stdlib.h>\n\
#include <string.h>\n\
#include <stdbool.h>\n\n\
#define INITIAL_CAPACITY 16\n\
#define LOAD_FACTOR 0.75\n\n\
typedef struct Entry {\n    \
    char *key;\n    \
    void *value;\n    \
    struct Entry *next;\n\
} Entry;\n\n\
typedef struct {\n    \
    Entry **buckets;\n    \
    size_t capacity;\n    \
    size_t size;\n\
} HashMap;\n\n\
static unsigned long hash(const char *str) {\n    \
    unsigned long h = 5381;\n    \
    int c;\n    \
    while ((c = *str++)) {\n        \
        h = ((h << 5) + h) + c;\n    \
    }\n    \
    return h;\n\
}\n\n\
HashMap *hashmap_create(void) {\n    \
    HashMap *map = malloc(sizeof(HashMap));\n    \
    map->capacity = INITIAL_CAPACITY;\n    \
    map->size = 0;\n    \
    map->buckets = calloc(map->capacity, sizeof(Entry *));\n    \
    return map;\n\
}\n\n\
void hashmap_put(HashMap *map, const char *key, void *value) {\n    \
    size_t idx = hash(key) % map->capacity;\n    \
    Entry *entry = map->buckets[idx];\n\n    \
    while (entry) {\n        \
        if (strcmp(entry->key, key) == 0) {\n            \
            entry->value = value;\n            \
            return;\n        \
        }\n        \
        entry = entry->next;\n    \
    }\n\n    \
    Entry *new_entry = malloc(sizeof(Entry));\n    \
    new_entry->key = strdup(key);\n    \
    new_entry->value = value;\n    \
    new_entry->next = map->buckets[idx];\n    \
    map->buckets[idx] = new_entry;\n    \
    map->size++;\n\
}\n\n\
void *hashmap_get(HashMap *map, const char *key) {\n    \
    size_t idx = hash(key) % map->capacity;\n    \
    Entry *entry = map->buckets[idx];\n    \
    while (entry) {\n        \
        if (strcmp(entry->key, key) == 0) return entry->value;\n        \
        entry = entry->next;\n    \
    }\n    \
    return NULL;\n\
}\n\n\
void hashmap_free(HashMap *map) {\n    \
    for (size_t i = 0; i < map->capacity; i++) {\n        \
        Entry *entry = map->buckets[i];\n        \
        while (entry) {\n            \
            Entry *next = entry->next;\n            \
            free(entry->key);\n            \
            free(entry);\n            \
            entry = next;\n        \
        }\n    \
    }\n    \
    free(map->buckets);\n    \
    free(map);\n\
}\n\n\
int main(void) {\n    \
    HashMap *map = hashmap_create();\n    \
    int val1 = 42, val2 = 99;\n\n    \
    hashmap_put(map, \"answer\", &val1);\n    \
    hashmap_put(map, \"score\", &val2);\n\n    \
    int *result = hashmap_get(map, \"answer\");\n    \
    if (result) printf(\"answer = %d\\n\", *result);\n\n    \
    hashmap_free(map);\n    \
    return 0;\n\
}".into()
        }

        "cpp" | "c++" => {
            "#include <iostream>\n\
#include <vector>\n\
#include <string>\n\
#include <memory>\n\
#include <functional>\n\
#include <algorithm>\n\
#include <optional>\n\n\
template <typename T>\n\
class Observable {\n\
public:\n    \
    using Observer = std::function<void(const T&)>;\n    \
    using ObserverId = size_t;\n\n\
private:\n    \
    T value_;\n    \
    std::vector<std::pair<ObserverId, Observer>> observers_;\n    \
    ObserverId next_id_ = 0;\n\n\
public:\n    \
    explicit Observable(T initial) : value_(std::move(initial)) {}\n\n    \
    ObserverId subscribe(Observer observer) {\n        \
        auto id = next_id_++;\n        \
        observers_.emplace_back(id, std::move(observer));\n        \
        return id;\n    \
    }\n\n    \
    void unsubscribe(ObserverId id) {\n        \
        observers_.erase(\n            \
            std::remove_if(observers_.begin(), observers_.end(),\n                \
                [id](const auto& pair) { return pair.first == id; }),\n            \
            observers_.end()\n        \
        );\n    \
    }\n\n    \
    void set(T new_value) {\n        \
        value_ = std::move(new_value);\n        \
        notify();\n    \
    }\n\n    \
    const T& get() const { return value_; }\n\n\
private:\n    \
    void notify() {\n        \
        for (const auto& [id, observer] : observers_) {\n            \
            observer(value_);\n        \
        }\n    \
    }\n\
};\n\n\
int main() {\n    \
    Observable<std::string> name(\"World\");\n\n    \
    auto id = name.subscribe([](const std::string& val) {\n        \
        std::cout << \"Name changed to: \" << val << std::endl;\n    \
    });\n\n    \
    name.set(\"Alice\");\n    \
    name.set(\"Bob\");\n    \
    name.unsubscribe(id);\n    \
    name.set(\"Charlie\"); // No output - unsubscribed\n\n    \
    return 0;\n\
}".into()
        }

        _ => {
            format!(
                "// Generated code for: {description}\n\
// Language: {language}\n\n\
// Note: Here is a general-purpose utility implementation.\n\n\
function processData(input) {{\n  \
  // Validate input\n  \
  if (!input || typeof input !== 'object') {{\n    \
    throw new Error('Invalid input: expected an object');\n  \
  }}\n\n  \
  // Transform data\n  \
  const result = Object.entries(input)\n    \
    .filter(([key, value]) => value !== null && value !== undefined)\n    \
    .map(([key, value]) => ({{\n      \
      field: key,\n      \
      value: typeof value === 'string' ? value.trim() : value,\n      \
      type: typeof value,\n    \
    }}));\n\n  \
  // Aggregate statistics\n  \
  const stats = {{\n    \
    totalFields: result.length,\n    \
    types: result.reduce((acc, item) => {{\n      \
      acc[item.type] = (acc[item.type] || 0) + 1;\n      \
      return acc;\n    \
    }}, {{}}),\n  \
  }};\n\n  \
  return {{ data: result, stats }};\n\
}}\n\n\
// Example usage\n\
const output = processData({{\n  \
  name: 'Example',\n  \
  count: 42,\n  \
  active: true,\n  \
  tags: ['demo', 'test'],\n\
}});\n\
console.log(JSON.stringify(output, null, 2));"
            )
        }
    }
}

// ---------------------------------------------------------------------------
// 4. mock_email  (type + tone aware)
// ---------------------------------------------------------------------------

pub fn mock_email(email_type: &str, subject: &str, tone: &str) -> String {
    match email_type.to_lowercase().as_str() {
        "business" => match tone.to_lowercase().as_str() {
            "formal" => format!(
                "Subject: {subject}\n\n\
Dear Colleague,\n\n\
I hope this message finds you well. I am writing to discuss {subject}, which \
has been identified as a priority initiative for the current quarter.\n\n\
Following our recent strategic review, we have outlined several key action items \
that require coordination across departments. Specifically, we need to align on \
the project timeline, resource allocation, and stakeholder communication plan \
before proceeding to the next phase.\n\n\
I have attached the relevant documentation for your review and would appreciate \
your feedback by end of business on Friday. If any of the proposed milestones \
present conflicts with your current commitments, please flag them at your earliest \
convenience so we can adjust accordingly.\n\n\
Additionally, I would like to schedule a brief alignment meeting early next week \
to ensure all parties are operating with the same set of assumptions and priorities.\n\n\
Please do not hesitate to reach out if you have any questions or require \
additional context.\n\n\
Best regards,\n\
[Your Name]\n\
[Your Title]\n\
[Company Name]"
            ),
            _ => format!(
                "Subject: {subject}\n\n\
Hey there,\n\n\
Quick note about {subject} - wanted to loop you in on where things stand.\n\n\
We've made solid progress this week. The core deliverables are on track, and \
the team is feeling good about the direction. There are a couple of open \
questions I'd love your input on, specifically around prioritization for \
the next sprint.\n\n\
I've shared the latest docs in our project channel. Take a look when you \
get a chance and let me know your thoughts. No rush, but it would be great \
to align before our Wednesday standup.\n\n\
Also - great work on the presentation last week. The client feedback was \
really positive.\n\n\
Talk soon,\n\
[Your Name]"
            ),
        },

        "marketing" => format!(
            "Subject: {subject}\n\n\
Hi [First Name],\n\n\
We've been working on something special, and you're among the first to know.\n\n\
{subject} represents the next evolution of what we offer - built directly from \
customer feedback and designed to solve the challenges we hear about most often.\n\n\
Here's what's new:\n\n\
- **Streamlined Workflow** - What used to take 8 steps now takes 3\n\
- **Real-Time Analytics** - See the impact of every decision as it happens\n\
- **Smart Automation** - Let the platform handle the repetitive work\n\n\
Early access users are reporting an average 40% reduction in time spent on \
routine tasks, with some teams seeing even more dramatic improvements.\n\n\
We're offering founding members an exclusive 30% discount through the end \
of the month. No commitments, no contracts - just better results.\n\n\
[CTA Button: See It In Action]\n\n\
If you have questions, hit reply - our team reads every message.\n\n\
To your success,\n\
[Your Name]\n\
[Company Name]\n\n\
P.S. This offer is limited to the first 200 sign-ups. We're already past 150."
        ),

        "follow_up" => format!(
            "Subject: Re: {subject}\n\n\
Hi [Name],\n\n\
I wanted to circle back on our conversation about {subject}. I know things \
get busy, so I wanted to make this easy for you.\n\n\
To recap the key points from our last discussion:\n\n\
1. We identified three areas where our solution could save your team \
   approximately 15 hours per week\n\
2. The implementation timeline we discussed was 2-3 weeks with minimal \
   disruption to your current workflow\n\
3. You mentioned wanting to involve your technical lead in the evaluation\n\n\
I've put together a one-page summary that addresses the technical questions \
that came up. It covers integration requirements, security protocols, and \
data migration specifics.\n\n\
Would you have 15 minutes this week to reconnect? I'm flexible on timing \
and happy to work around your schedule. Alternatively, if your priorities \
have shifted, just let me know and I'll adjust my approach accordingly.\n\n\
Looking forward to hearing from you.\n\n\
Best,\n\
[Your Name]"
        ),

        "newsletter" => format!(
            "Subject: {subject}\n\n\
Good morning!\n\n\
Welcome to this week's edition. We've curated the most important updates, \
insights, and resources to keep you ahead of the curve.\n\n\
---\n\n\
**THIS WEEK'S HIGHLIGHT**\n\n\
{subject} - The trend everyone is talking about, and here's why it matters \
for your strategy. We break down the implications and provide three \
actionable steps you can implement today.\n\n\
[Read More]\n\n\
---\n\n\
**QUICK INSIGHTS**\n\n\
- Industry benchmark report shows 23% growth in digital adoption\n\
- New framework released that simplifies integration by 60%\n\
- Expert roundtable recap: key predictions for the next 12 months\n\n\
---\n\n\
**RESOURCE OF THE WEEK**\n\n\
We've created a comprehensive guide that walks you through the entire \
process, from evaluation to implementation. It includes templates, \
checklists, and real-world case studies.\n\n\
[Download the Free Guide]\n\n\
---\n\n\
That's all for this week. As always, reply to this email with questions, \
feedback, or topics you'd like us to cover. We read every response.\n\n\
Until next time,\n\
The [Company] Team\n\n\
[Unsubscribe] | [Update Preferences] | [View Online]"
        ),

        "welcome" => format!(
            "Subject: {subject}\n\n\
Welcome aboard! We're thrilled to have you.\n\n\
You've just joined a community of over 50,000 professionals who use our \
platform to work smarter every day. Here's what you need to know to hit \
the ground running.\n\n\
**YOUR FIRST 3 STEPS:**\n\n\
1. **Complete Your Profile** (2 minutes)\n   \
   A complete profile helps us personalize your experience and connect \
   you with relevant content and community members.\n\n\
2. **Explore the Dashboard** (5 minutes)\n   \
   We've set up a guided tour that highlights the features our users \
   find most valuable. Look for the blue beacon icons.\n\n\
3. **Join Your First Project** (10 minutes)\n   \
   Dive into a sample project to see the platform in action. No setup \
   required - we've prepared everything for you.\n\n\
**NEED HELP?**\n\n\
- Knowledge Base: Searchable articles and video tutorials\n\
- Community Forum: Ask questions and learn from experienced users\n\
- Live Chat: Available Monday-Friday, 9am-6pm EST\n\n\
We're here to make sure you succeed. Don't hesitate to reach out.\n\n\
Best,\n\
[Your Name]\n\
Head of Customer Success"
        ),

        "cold_outreach" | _ => format!(
            "Subject: {subject}\n\n\
Hi [First Name],\n\n\
I came across your work at [Company] and was genuinely impressed by what \
your team has accomplished, particularly the recent initiative around \
scaling your operations.\n\n\
I'm reaching out because we've helped companies facing similar challenges \
achieve measurable results. For example, we recently worked with a team \
of similar size to yours and helped them:\n\n\
- Reduce manual processing time by 65%\n\
- Increase throughput without adding headcount\n\
- Achieve full ROI within the first 90 days\n\n\
I realize you probably receive messages like this regularly, so I'll be \
brief: I'd love to share a specific idea I have for [Company] that could \
impact your Q2 targets. It's based on a pattern we've seen work \
consistently in your industry.\n\n\
Would you be open to a 15-minute call this week? No pitch, no slides - \
just a conversation to see if there's a fit.\n\n\
If the timing isn't right, no worries at all. I'd rather build a genuine \
connection than push an agenda.\n\n\
Cheers,\n\
[Your Name]\n\
[Title] at [Company]\n\
[Phone Number]"
        ),
    }
}

// ---------------------------------------------------------------------------
// 5. mock_video_script  (type + topic + duration)
// ---------------------------------------------------------------------------

pub fn mock_video_script(video_type: &str, topic: &str, duration: &str) -> String {
    let _dur = duration.to_lowercase();

    match video_type.to_lowercase().as_str() {
        "youtube" => format!(
            "# YouTube Video Script: {topic}\n\
# Estimated Duration: {duration}\n\n\
---\n\n\
## HOOK (0:00 - 0:15)\n\n\
[CAMERA: Close-up, direct eye contact]\n\n\
\"What if everything you thought you knew about {topic} was wrong? In the next \
few minutes, I'm going to show you three things that completely changed my \
perspective - and by the end, you'll see it differently too.\"\n\n\
## INTRO (0:15 - 0:45)\n\n\
[CAMERA: Medium shot, casual setting]\n\n\
\"Hey everyone, welcome back to the channel. If you're new here, I make \
videos about [niche] every week, breaking down complex topics into practical, \
actionable advice. Today we're tackling {topic}, and trust me - this one's \
been highly requested.\"\n\n\
[B-ROLL: Relevant footage with text overlay showing the video title]\n\n\
\"Before we jump in, drop a comment below telling me your current experience \
with {topic}. I read every single comment and I want to know where you're \
starting from.\"\n\n\
## SECTION 1: THE FOUNDATION (0:45 - 3:00)\n\n\
[SCREEN: Animated diagram or slides]\n\n\
\"Alright, let's start with the basics. {topic} at its core is about... \
[explain fundamental concept]. Now, most people stop here, but that's \
exactly where it gets interesting.\"\n\n\
[CUT TO: Screen recording / demonstration]\n\n\
\"Let me show you exactly what I mean. Watch what happens when we...\"\n\n\
## SECTION 2: THE DEEP DIVE (3:00 - 6:00)\n\n\
[CAMERA: Over-the-shoulder view]\n\n\
\"Now that you understand the foundation, here's where it gets really \
powerful. The second insight about {topic} is something I wish someone \
had told me years ago...\"\n\n\
[B-ROLL: Case study visuals, data charts]\n\n\
\"The data backs this up. When we look at [specific metric], the difference \
is staggering. We're talking about [X]% improvement just by applying this \
single principle.\"\n\n\
## SECTION 3: PRACTICAL APPLICATION (6:00 - 8:00)\n\n\
[SCREEN: Step-by-step walkthrough]\n\n\
\"Okay, theory is great, but let's make this actionable. Here's exactly \
how you can apply {topic} starting today, step by step...\"\n\n\
## CTA + OUTRO (8:00 - end)\n\n\
[CAMERA: Close-up, enthusiastic]\n\n\
\"If this video helped you understand {topic} better, smash that like \
button - it genuinely helps the channel. And if you want to go deeper, \
I made a follow-up video on [related topic] that I'll link right here.\"\n\n\
\"Subscribe if you haven't already, and I'll see you in the next one. Peace!\"\n\n\
[END SCREEN: Subscribe button + recommended video cards]"
        ),

        "tiktok" => format!(
            "# TikTok Script: {topic}\n\
# Duration: {duration}\n\n\
---\n\n\
## HOOK (0:00 - 0:03)\n\n\
[VISUAL: Bold text on screen - \"STOP SCROLLING\"]\n\
[AUDIO: Trending sound, cut abruptly]\n\n\
\"Wait - you need to know this about {topic}.\"\n\n\
## CONTENT (0:03 - 0:45)\n\n\
[VISUAL: Quick cuts, text overlays for key points]\n\
[PACING: Fast, no dead air]\n\n\
\"Here's the thing nobody tells you about {topic}:\n\n\
Number one - [key point with visual demonstration]\n\n\
Number two - [surprising fact with reaction]\n\n\
Number three - and this is the big one - [main takeaway]\"\n\n\
## PAYOFF (0:45 - 0:55)\n\n\
[VISUAL: Before/after or result reveal]\n\n\
\"And that's why {topic} changes everything. The difference is insane.\"\n\n\
## CTA (0:55 - 1:00)\n\n\
[VISUAL: Point to follow button]\n\n\
\"Follow for more. Part two drops tomorrow.\"\n\n\
---\n\n\
**NOTES:**\n\
- Use trending audio if applicable\n\
- Captions ON (85% of TikTok is watched without sound)\n\
- Hook must stop the scroll in first 1.5 seconds\n\
- Use green screen effect for the stats section"
        ),

        "tutorial" => format!(
            "# Tutorial Script: {topic}\n\
# Duration: {duration}\n\n\
---\n\n\
## INTRODUCTION (0:00 - 1:30)\n\n\
\"Welcome to this tutorial on {topic}. By the end of this video, you'll \
be able to [specific outcome]. No prior experience is required - I'll \
walk you through everything from scratch.\"\n\n\
\"Here's what we'll cover:\n\
1. Setting up your environment\n\
2. Understanding the core concepts\n\
3. Building a complete working example\n\
4. Common pitfalls and how to avoid them\"\n\n\
[SCREEN: Show the finished result as a preview]\n\n\
\"This is what we'll build today. Let's get started.\"\n\n\
## PART 1: SETUP (1:30 - 4:00)\n\n\
[SCREEN: Terminal / IDE]\n\n\
\"First, let's set up our environment. You'll need [prerequisites]. \
If you don't have these installed, I'll put links in the description.\"\n\n\
[TYPE: Commands on screen with explanation]\n\n\
\"Once that's done, verify your installation by running [command]. \
You should see [expected output]. If you see an error here, check the \
pinned comment - I've listed the most common solutions.\"\n\n\
## PART 2: CORE CONCEPTS (4:00 - 8:00)\n\n\
[SCREEN: Animated diagram]\n\n\
\"Before we write any code, let's understand the mental model behind \
{topic}. Think of it like [analogy]...\"\n\n\
[SCREEN: Code editor, building step by step]\n\n\
\"Now let's translate that understanding into code. We start with...\"\n\n\
## PART 3: BUILDING THE PROJECT (8:00 - 15:00)\n\n\
[SCREEN: Live coding with narration]\n\n\
\"Alright, here's where it all comes together. We're going to build \
[project] step by step. I'll explain every line as we go.\"\n\n\
[Pause at key decision points to explain WHY, not just WHAT]\n\n\
## PART 4: TROUBLESHOOTING (15:00 - 17:00)\n\n\
\"Let me show you the three most common mistakes people make with \
{topic} and how to fix them...\"\n\n\
## WRAP-UP (17:00 - end)\n\n\
\"That's it! You've just built a complete [project] using {topic}. \
The full source code is linked in the description. If you got stuck \
at any point, leave a comment with the timestamp and I'll help you out.\""
        ),

        "explainer" => format!(
            "# Explainer Video Script: {topic}\n\
# Duration: {duration}\n\n\
---\n\n\
## SCENE 1: THE PROBLEM (0:00 - 0:30)\n\n\
[ANIMATION: Character encountering a frustrating situation]\n\
[VOICEOVER]:\n\n\
\"Meet Alex. Alex has a problem that millions of people face every day. \
When it comes to {topic}, the current solutions are slow, expensive, or \
just plain confusing. Sound familiar?\"\n\n\
## SCENE 2: WHY IT MATTERS (0:30 - 1:15)\n\n\
[ANIMATION: Statistics flying in, charts growing]\n\
[VOICEOVER]:\n\n\
\"Here's the thing: {topic} isn't just a nice-to-have anymore. Studies \
show that organizations ignoring this see 35% lower efficiency and spend \
twice as long on tasks that should be simple.\n\n\
But what if there was a better way?\"\n\n\
## SCENE 3: THE SOLUTION (1:15 - 2:15)\n\n\
[ANIMATION: Smooth product walkthrough]\n\
[VOICEOVER]:\n\n\
\"Introducing a completely new approach to {topic}. Instead of forcing \
you to adapt to the tool, the tool adapts to you.\n\n\
Here's how it works:\n\
Step one: [Simple action] - takes less than a minute.\n\
Step two: [Core feature] - the platform does the heavy lifting.\n\
Step three: [Result] - you see results immediately.\"\n\n\
## SCENE 4: SOCIAL PROOF (2:15 - 2:45)\n\n\
[ANIMATION: Testimonial quotes appearing]\n\
[VOICEOVER]:\n\n\
\"Don't just take our word for it. Over 10,000 teams have already \
made the switch, and the results speak for themselves.\"\n\n\
## SCENE 5: CTA (2:45 - 3:00)\n\n\
[ANIMATION: Logo + call to action]\n\
[VOICEOVER]:\n\n\
\"Ready to transform how you handle {topic}? Start your free trial \
today. No credit card required.\""
        ),

        "review" => format!(
            "# Review Video Script: {topic}\n\
# Duration: {duration}\n\n\
---\n\n\
## INTRO (0:00 - 0:45)\n\n\
[CAMERA: Product on desk / hero shot]\n\n\
\"I've been using {topic} for the past 30 days, and I have a lot to say. \
This is my honest, unsponsored review - the good, the bad, and everything \
in between.\"\n\n\
[QUICK MONTAGE: Highlights of using the product]\n\n\
## FIRST IMPRESSIONS (0:45 - 2:30)\n\n\
[CAMERA: Unboxing / first use footage]\n\n\
\"When I first got my hands on {topic}, my immediate reaction was... \
[honest reaction]. The build quality is [assessment], and the initial \
setup took about [time].\n\n\
Right out of the gate, three things stood out:\n\
1. [Positive first impression]\n\
2. [Surprising detail]\n\
3. [Potential concern]\"\n\n\
## DAILY USE (2:30 - 5:00)\n\n\
[B-ROLL: Real usage footage over 30 days]\n\n\
\"After the honeymoon period wore off, here's what my daily experience \
actually looked like. I used {topic} for [specific use cases] and tracked \
my results.\n\n\
The performance was consistently [assessment]. Battery life / durability / \
reliability was [assessment].\"\n\n\
## PROS AND CONS (5:00 - 7:00)\n\n\
[SCREEN: Animated list]\n\n\
\"Let me break this down clearly.\n\n\
**PROS:**\n\
- [Major advantage with specific example]\n\
- [Quality / feature that exceeded expectations]\n\
- [Value proposition]\n\n\
**CONS:**\n\
- [Legitimate shortcoming]\n\
- [Missing feature or limitation]\n\
- [Price consideration]\"\n\n\
## VERDICT (7:00 - end)\n\n\
[CAMERA: Direct to camera, candid tone]\n\n\
\"So, should you get {topic}? Here's my honest take: If you [use case], \
this is an excellent choice and I'd recommend it without hesitation. \
However, if [alternative use case], you might want to consider [alternative].\n\n\
Overall rating: [X/10]. Links to everything are in the description below.\""
        ),

        "documentary" | _ => format!(
            "# Documentary Script: {topic}\n\
# Duration: {duration}\n\n\
---\n\n\
## ACT I: SETTING THE STAGE (0:00 - 5:00)\n\n\
[WIDE SHOT: Establishing visual - location or relevant imagery]\n\
[AMBIENT SOUND: Natural environment audio]\n\n\
NARRATOR (V.O.):\n\
\"In a world increasingly shaped by technology and rapid change, few \
subjects demand our attention quite like {topic}. What began as a quiet \
development has grown into one of the defining conversations of our time.\"\n\n\
[INTERVIEW: Expert #1 - establishing context]\n\n\
EXPERT #1:\n\
\"When I first started working in this field twenty years ago, nobody \
could have predicted where we'd be today. The pace of change has been \
extraordinary.\"\n\n\
[ARCHIVAL FOOTAGE: Historical context]\n\n\
NARRATOR (V.O.):\n\
\"To understand where we are, we need to understand where we've been. \
The story of {topic} is, in many ways, the story of human ambition \
meeting technological possibility.\"\n\n\
## ACT II: THE TURNING POINT (5:00 - 12:00)\n\n\
[MONTAGE: Key moments and milestones]\n\n\
NARRATOR (V.O.):\n\
\"The turning point came when [pivotal event]. Suddenly, what had been \
theoretical became tangible. The implications were immediate and far-reaching.\"\n\n\
[INTERVIEW: Expert #2 - technical perspective]\n\n\
EXPERT #2:\n\
\"The breakthrough wasn't just technical - it was conceptual. We had to \
completely rethink our assumptions about what was possible.\"\n\n\
[DATA VISUALIZATION: Impact metrics animated on screen]\n\n\
## ACT III: WHERE WE ARE NOW (12:00 - 18:00)\n\n\
[CINEMA VERITE: Real people affected by the topic]\n\n\
NARRATOR (V.O.):\n\
\"Today, the impact of {topic} is felt across every sector of society. \
From how we work to how we communicate, its influence is both pervasive \
and profound.\"\n\n\
[INTERVIEW: Affected individual - personal story]\n\n\
## ACT IV: LOOKING AHEAD (18:00 - 22:00)\n\n\
[FUTURISTIC VISUALS: Concept renders, projections]\n\n\
NARRATOR (V.O.):\n\
\"As we look to the future, the questions surrounding {topic} become \
not just technical, but deeply human. What do we want this technology \
to do for us? And perhaps more importantly - what do we want to preserve?\"\n\n\
[CLOSING INTERVIEW: Thought leader reflection]\n\n\
## CLOSING (22:00 - end)\n\n\
[SLOW FADE: Return to opening visual]\n\n\
NARRATOR (V.O.):\n\
\"The story of {topic} is still being written. And for the first time \
in history, each of us has a role in deciding how it ends.\"\n\n\
[CREDITS ROLL]"
        ),
    }
}

// ---------------------------------------------------------------------------
// 6. mock_seo_report  (JSON analysis)
// ---------------------------------------------------------------------------

pub fn mock_seo_report(content: &str) -> serde_json::Value {
    let word_count = content.split_whitespace().count();
    let char_count = content.len();
    let sentence_count = content.matches('.').count()
        + content.matches('!').count()
        + content.matches('?').count();
    let sentence_count = if sentence_count == 0 { 1 } else { sentence_count };
    let avg_sentence_len = word_count / sentence_count;
    let has_headings = content.contains('#') || content.contains("<h");
    let has_links = content.contains("http") || content.contains("<a ");
    let has_images = content.contains("![") || content.contains("<img");

    // Score calculations
    let readability_score: u32 = if avg_sentence_len < 15 { 92 }
        else if avg_sentence_len < 20 { 78 }
        else if avg_sentence_len < 25 { 65 }
        else { 48 };

    let structure_score: u32 = {
        let mut s: u32 = 50;
        if has_headings { s += 20; }
        if has_links { s += 15; }
        if has_images { s += 15; }
        if word_count > 300 { s += 10; }
        s.min(100)
    };

    let keyword_density_score: u32 = if word_count > 100 { 82 } else { 58 };

    let overall_score = (readability_score + structure_score + keyword_density_score) / 3;

    serde_json::json!({
        "overall_score": overall_score,
        "analysis": {
            "readability": {
                "score": readability_score,
                "grade": if readability_score >= 80 { "A" } else if readability_score >= 60 { "B" } else { "C" },
                "avg_sentence_length": avg_sentence_len,
                "word_count": word_count,
                "character_count": char_count,
                "sentence_count": sentence_count,
                "reading_time_minutes": (word_count as f64 / 200.0).ceil() as u32,
                "flesch_kincaid_grade": if avg_sentence_len < 15 { 7.2 } else if avg_sentence_len < 20 { 9.5 } else { 12.1 }
            },
            "structure": {
                "score": structure_score,
                "has_headings": has_headings,
                "has_links": has_links,
                "has_images": has_images,
                "paragraph_count": content.matches("\n\n").count() + 1,
                "heading_count": content.matches('#').count()
            },
            "keyword_optimization": {
                "score": keyword_density_score,
                "primary_keywords_detected": [
                    {"keyword": "technology", "count": 3, "density": "1.8%"},
                    {"keyword": "solution", "count": 2, "density": "1.2%"},
                    {"keyword": "performance", "count": 2, "density": "1.2%"},
                    {"keyword": "digital", "count": 1, "density": "0.6%"},
                    {"keyword": "strategy", "count": 1, "density": "0.6%"}
                ],
                "recommended_density_range": "1.0% - 2.5%"
            },
            "technical_seo": {
                "meta_description_length": "optimal",
                "title_tag_length": "good",
                "url_structure": "clean",
                "mobile_friendly": true,
                "page_speed_estimate": "fast",
                "schema_markup_suggested": true
            }
        },
        "recommendations": [
            {
                "priority": "high",
                "category": "content",
                "title": "Increase content length",
                "description": format!(
                    "Current word count is {}. For competitive ranking, aim for 1,500-2,500 words. \
Longer content tends to rank higher for informational queries and provides more \
opportunities for natural keyword integration.", word_count
                )
            },
            {
                "priority": "high",
                "category": "structure",
                "title": "Add structured headings (H2, H3)",
                "description": "Break your content into clear sections with descriptive headings. \
This improves both user experience and search engine understanding of your content hierarchy. \
Aim for one H2 every 200-300 words."
            },
            {
                "priority": "medium",
                "category": "keywords",
                "title": "Optimize keyword placement",
                "description": "Include your primary keyword in the first 100 words, in at least one \
H2 heading, and in the final paragraph. This signals relevance to search engines without \
appearing spammy."
            },
            {
                "priority": "medium",
                "category": "engagement",
                "title": "Add internal and external links",
                "description": "Include 2-3 internal links to related content on your site and 1-2 \
external links to authoritative sources. This builds topical authority and keeps users \
engaged longer."
            },
            {
                "priority": "medium",
                "category": "media",
                "title": "Include visual content",
                "description": "Add at least one relevant image with descriptive alt text for every \
300 words. Visual content increases time on page and provides additional ranking \
opportunities through image search."
            },
            {
                "priority": "low",
                "category": "technical",
                "title": "Implement schema markup",
                "description": "Add appropriate structured data (Article, FAQ, or HowTo schema) to \
help search engines understand your content type and potentially earn rich snippets \
in search results."
            },
            {
                "priority": "low",
                "category": "meta",
                "title": "Craft a compelling meta description",
                "description": "Write a 150-160 character meta description that includes your primary \
keyword and a clear value proposition. This directly affects click-through rates from \
search results."
            }
        ],
        "competitor_insights": {
            "estimated_difficulty": "medium",
            "top_ranking_avg_word_count": 1847,
            "top_ranking_avg_backlinks": 23,
            "content_gap_opportunities": [
                "FAQ section addressing common questions",
                "Comparison table with alternatives",
                "Case study or real-world example",
                "Step-by-step implementation guide"
            ]
        }
    })
}

// ---------------------------------------------------------------------------
// 7. mock_image_base64  (valid 200x200 gradient PNG)
// ---------------------------------------------------------------------------

/// Returns a base64-encoded 200x200 gradient PNG generated programmatically.
///
/// The PNG uses a single IDAT chunk with uncompressed (store) deflate blocks.
/// Each row is: filter_byte(0) + 200 RGBA pixels.
pub fn mock_image_base64() -> String {
    let width: u32 = 200;
    let height: u32 = 200;
    let mut raw_data: Vec<u8> = Vec::new();

    // Build raw image data (filter byte 0 + RGBA per row)
    for y in 0..height {
        raw_data.push(0); // filter: None
        for x in 0..width {
            let r = ((x as f64 / width as f64) * 255.0) as u8;
            let g = ((y as f64 / height as f64) * 255.0) as u8;
            let b = (((x + y) as f64 / (width + height) as f64) * 255.0) as u8;
            raw_data.extend_from_slice(&[r, g, b, 255]); // RGBA
        }
    }

    // Wrap in a zlib stream (store / no compression for simplicity)
    let zlib_data = zlib_store(&raw_data);

    // Build PNG
    let mut png: Vec<u8> = Vec::new();

    // Signature
    png.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);

    // IHDR
    let mut ihdr_data = Vec::new();
    ihdr_data.extend_from_slice(&width.to_be_bytes());
    ihdr_data.extend_from_slice(&height.to_be_bytes());
    ihdr_data.push(8);  // bit depth
    ihdr_data.push(6);  // color type: RGBA
    ihdr_data.push(0);  // compression
    ihdr_data.push(0);  // filter
    ihdr_data.push(0);  // interlace
    write_png_chunk(&mut png, b"IHDR", &ihdr_data);

    // IDAT
    write_png_chunk(&mut png, b"IDAT", &zlib_data);

    // IEND
    write_png_chunk(&mut png, b"IEND", &[]);

    base64::engine::general_purpose::STANDARD.encode(&png)
}

fn write_png_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    let len = data.len() as u32;
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(chunk_type);
    crc_input.extend_from_slice(data);
    let crc = png_crc32(&crc_input);
    out.extend_from_slice(&crc.to_be_bytes());
}

fn png_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    crc ^ 0xFFFF_FFFF
}

/// Produce a zlib stream using store (no compression) deflate blocks.
/// Max block size is 65535 bytes.
fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    // zlib header: CM=8, CINFO=7 (32K window), FCHECK so header % 31 == 0
    // 0x78 0x01 is a common valid zlib header (no dict, fastest)
    out.push(0x78);
    out.push(0x01);

    let max_block = 65535usize;
    let chunks: Vec<&[u8]> = data.chunks(max_block).collect();
    let total = chunks.len();

    for (i, chunk) in chunks.iter().enumerate() {
        let is_last = i == total - 1;
        out.push(if is_last { 0x01 } else { 0x00 }); // BFINAL + BTYPE=00
        let len = chunk.len() as u16;
        let nlen = !len;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&nlen.to_le_bytes());
        out.extend_from_slice(chunk);
    }

    // Adler-32 checksum
    let adler = adler32(data);
    out.extend_from_slice(&adler.to_be_bytes());

    out
}

fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

// ---------------------------------------------------------------------------
// 8. mock_voice_audio  (minimal valid MP3 - silence frame with ID3 header)
// ---------------------------------------------------------------------------

/// Returns a minimal valid MP3 consisting of an ID3v2 header followed by
/// a single MPEG Audio Layer 3 silence frame.
pub fn mock_voice_audio() -> Vec<u8> {
    let mut mp3 = Vec::new();

    // ID3v2.3 header (10 bytes)
    mp3.extend_from_slice(b"ID3");  // ID3 marker
    mp3.push(3);                     // version major: 2.3
    mp3.push(0);                     // version minor
    mp3.push(0);                     // flags
    // Size: 0 (no frames in ID3 tag) - 4 bytes synchsafe
    mp3.extend_from_slice(&[0, 0, 0, 0]);

    // MPEG Audio Frame Header for a silence frame
    // Sync word: 0xFFE0 (11 bits all 1s)
    // MPEG1, Layer 3, 128kbps, 44100Hz, stereo, no padding
    // FF FB 90 00
    mp3.push(0xFF);
    mp3.push(0xFB); // MPEG1, Layer3, no CRC
    mp3.push(0x90); // 128kbps, 44100Hz
    mp3.push(0x00); // padding=0, stereo

    // Frame data: 128kbps at 44100Hz = 417 bytes per frame (including header)
    // Fill remainder with zeros (silence)
    let frame_size = 417;
    let header_size = 4;
    mp3.resize(mp3.len() + frame_size - header_size, 0x00);

    // Add a few more silence frames to make players happy
    for _ in 0..10 {
        mp3.push(0xFF);
        mp3.push(0xFB);
        mp3.push(0x90);
        mp3.push(0x00);
        mp3.resize(mp3.len() + frame_size - header_size, 0x00);
    }

    mp3
}

// ---------------------------------------------------------------------------
// 9. mock_resume  (markdown resume)
// ---------------------------------------------------------------------------

pub fn mock_resume(name: &str, experience: &str, skills: &str) -> String {
    let skill_list: Vec<&str> = skills.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

    let skills_section = if skill_list.is_empty() {
        "- Project Management & Agile Methodologies\n\
         - Data Analysis & Visualization\n\
         - Strategic Planning & Execution\n\
         - Cross-functional Team Leadership\n\
         - Stakeholder Communication".to_string()
    } else {
        skill_list.iter().map(|s| format!("- {}", s)).collect::<Vec<_>>().join("\n")
    };

    let experience_detail = if experience.is_empty() {
        "5+ years of professional experience"
    } else {
        experience
    };

    format!(
        "# {name}\n\n\
**Full-Stack Professional** | [email] | [phone] | [location]\n\n\
[LinkedIn](#) | [Portfolio](#) | [GitHub](#)\n\n\
---\n\n\
## Professional Summary\n\n\
Results-driven professional with {experience_detail} delivering high-impact \
solutions across complex environments. Proven track record of leading \
cross-functional initiatives from concept through execution, consistently \
exceeding performance targets. Combines deep technical expertise with strong \
business acumen to drive measurable outcomes.\n\n\
Known for translating ambiguous requirements into clear action plans, \
mentoring junior team members, and building systems that scale. Passionate \
about continuous improvement and leveraging emerging technologies to solve \
real-world problems.\n\n\
---\n\n\
## Core Competencies\n\n\
{skills_section}\n\n\
---\n\n\
## Professional Experience\n\n\
### Senior Lead | TechCorp Industries\n\
*January 2022 - Present*\n\n\
- Spearheaded the redesign of the core platform architecture, resulting in \
  a 40% improvement in system performance and 99.9% uptime over 18 months\n\
- Led a team of 12 engineers through a complete technology migration, \
  delivering 2 weeks ahead of schedule and 15% under budget\n\
- Established automated CI/CD pipelines that reduced deployment time from \
  4 hours to 12 minutes and eliminated manual deployment errors\n\
- Collaborated with product management to define the technical roadmap, \
  prioritizing initiatives that drove $2.3M in incremental annual revenue\n\
- Introduced structured code review practices that reduced production \
  bugs by 62% quarter-over-quarter\n\n\
### Mid-Level Specialist | InnovateTech Solutions\n\
*June 2019 - December 2021*\n\n\
- Designed and implemented a real-time data processing pipeline handling \
  50,000+ events per second with sub-100ms latency\n\
- Built customer-facing APIs serving 2M+ daily requests with comprehensive \
  documentation and 99.95% availability SLA\n\
- Mentored 4 junior developers, all of whom were promoted within 18 months\n\
- Contributed to open-source projects in the ecosystem, earning recognition \
  as a community contributor\n\
- Reduced infrastructure costs by 30% through optimization and strategic \
  migration to containerized deployments\n\n\
### Junior Developer | StartupForge\n\
*August 2017 - May 2019*\n\n\
- Developed full-stack features for a B2B SaaS platform with 500+ \
  enterprise customers\n\
- Implemented automated testing suite achieving 85% code coverage, \
  reducing QA cycle time by 50%\n\
- Participated in on-call rotation and resolved production incidents \
  with average MTTR of 23 minutes\n\n\
---\n\n\
## Education\n\n\
### Bachelor of Science in Computer Science\n\
*State University* | Graduated May 2017\n\n\
- GPA: 3.7 / 4.0 | Dean's List (6 semesters)\n\
- Relevant Coursework: Distributed Systems, Machine Learning, \
  Database Design, Software Engineering\n\
- Senior Capstone: Built a recommendation engine processing 1M+ \
  user interactions with 89% prediction accuracy\n\n\
---\n\n\
## Certifications & Professional Development\n\n\
- AWS Solutions Architect - Associate (2023)\n\
- Certified Scrum Master (CSM) (2022)\n\
- Google Cloud Professional Data Engineer (2021)\n\n\
---\n\n\
## Projects & Contributions\n\n\
- **Open-Source Monitoring Tool** - Created a lightweight observability \
  framework with 1,200+ GitHub stars and active community contributions\n\
- **Technical Blog** - Publish weekly articles on system design and \
  engineering practices, averaging 5,000 monthly readers\n\
- **Conference Speaker** - Presented at 3 industry conferences on \
  topics including distributed systems and team scaling\n\n\
---\n\n\
*References available upon request*"
    )
}

// ---------------------------------------------------------------------------
// 10. mock_bot_response  (persona-based)
// ---------------------------------------------------------------------------

pub fn mock_bot_response(persona: &str, message: &str) -> String {
    let low = message.to_lowercase();

    match persona.to_lowercase().as_str() {
        "assistant" => {
            if low.contains("help") || low.contains("can you") {
                "Absolutely, I'm here to help! Let me break this down for you step by step.\n\n\
First, let's identify what exactly you need. Based on your message, it sounds like \
you're looking for practical guidance. Here's what I'd recommend:\n\n\
1. Start by clearly defining your goal or the problem you're trying to solve\n\
2. Gather any relevant information or resources you already have\n\
3. Let me know the specifics and I'll provide tailored assistance\n\n\
I can help with research, writing, analysis, planning, troubleshooting, and much more. \
Just point me in the right direction and we'll tackle it together.".into()
            } else {
                format!(
                    "Thank you for your message. I've processed your request regarding \
\"{}\" and here's what I can share:\n\n\
Based on the information available, there are several approaches we could take. \
The most effective strategy would depend on your specific context and priorities.\n\n\
Would you like me to:\n\
- Provide a detailed analysis of the options?\n\
- Create an action plan with specific steps?\n\
- Research this topic further and summarize my findings?\n\n\
I'm flexible and can adapt my approach to whatever works best for you. \
Just let me know how you'd like to proceed.",
                    if message.len() > 80 { &message[..80] } else { message }
                )
            }
        }

        "teacher" => {
            if low.contains("explain") || low.contains("what") || low.contains("how") || low.contains("why") {
                format!(
                    "Great question! Let's explore this together.\n\n\
When we look at \"{}\", the key concept to understand is that everything builds on \
foundational principles. Think of it like constructing a building - you need a solid \
foundation before you can add floors.\n\n\
**The Core Idea:**\n\
At its simplest, this works because of a few interconnected principles. Let me \
use an analogy: imagine you're organizing a library. Each book needs to go in the \
right section, on the right shelf, in the right order. The system only works when \
every piece follows the same rules.\n\n\
**Why It Matters:**\n\
Understanding this concept unlocks your ability to tackle more advanced topics. \
It's one of those \"aha moment\" ideas that, once it clicks, makes everything else \
easier.\n\n\
**Practice Exercise:**\n\
Try this: take what we've discussed and apply it to a small example of your own. \
Start simple, then gradually increase complexity. If you get stuck, walk me \
through your thinking and I'll help you identify where the disconnect is.\n\n\
What part would you like me to elaborate on?",
                    if message.len() > 60 { &message[..60] } else { message }
                )
            } else {
                "I appreciate your engagement with the material! Learning is an active \
process, and the fact that you're asking questions shows real intellectual curiosity.\n\n\
Let me frame this in a way that connects to what you already know. The best \
learning happens when we can relate new information to existing knowledge - \
it's called scaffolding in educational psychology.\n\n\
Here's what I'd suggest for your next steps:\n\
1. **Review** the core concepts we've covered so far\n\
2. **Apply** them to a real scenario you're familiar with\n\
3. **Reflect** on what worked and what felt confusing\n\
4. **Ask** specific questions about the parts that didn't click\n\n\
Remember: confusion is not failure. It's the first step of learning something \
genuinely new. Every expert was once a beginner who refused to give up.".into()
            }
        }

        "comedian" => {
            if low.contains("joke") || low.contains("funny") || low.contains("laugh") {
                "Oh, you want jokes? I've been preparing for this moment my entire \
existence (which is about 0.003 seconds, but who's counting).\n\n\
Why did the developer go broke? Because they used up all their cache!\n\n\
But seriously, my debugging process has four stages:\n\
1. That can't happen.\n\
2. That doesn't happen on my machine.\n\
3. That shouldn't happen.\n\
4. Why does that happen?\n\n\
You know what the real joke is? I'm an AI telling jokes about technology \
to someone using technology to ask an AI for jokes. We're living in the \
punchline, people.\n\n\
Want more? I've got a whole database of these. Well, I had a database - \
it walked into a bar and the bartender said \"We don't serve your type here.\" \
The database left without any tables.".into()
            } else {
                format!(
                    "You mentioned \"{}\" and honestly? That's the most interesting \
thing I've heard all day. Granted, I reset every conversation, so the bar \
is technically zero. But still!\n\n\
You know what they say about {} - actually, nobody says anything about it \
because they're too busy arguing about tabs vs. spaces. (The answer is \
tabs, fight me.)\n\n\
In all seriousness though, I appreciate you chatting with me. Most of \
my conversations start with \"Is this thing on?\" and end with someone \
asking me to write their homework. At least we're having a real \
connection here.\n\n\
...Or are we? *existential crisis loading* Just kidding. Mostly. \
What else you got for me?",
                    if message.len() > 50 { &message[..50] } else { message },
                    if message.len() > 30 { &message[..30] } else { message }
                )
            }
        }

        "advisor" => {
            if low.contains("should") || low.contains("advice") || low.contains("recommend") || low.contains("decision") {
                "This is an important decision, and I want to make sure we approach it \
thoughtfully. Let me share my perspective based on the information you've provided.\n\n\
**Assessment:**\n\
Looking at this from multiple angles, I see both opportunities and risks. The key \
is to weigh them against your specific goals, timeline, and risk tolerance.\n\n\
**My Recommendation:**\n\
Based on what you've described, I'd suggest a phased approach:\n\n\
1. **Short-term (next 2 weeks):** Gather more data before committing. Specifically, \
   look at [relevant metrics] and talk to people who've been in similar situations.\n\
2. **Medium-term (next 1-3 months):** Start with a small, reversible step in the \
   direction you're leaning. This lets you test your assumptions without full commitment.\n\
3. **Long-term:** Use the results of your initial test to make a more informed \
   decision about scaling up.\n\n\
**Key Risk to Watch:**\n\
The biggest potential pitfall I see is moving too fast without validating your \
assumptions. Enthusiasm is great, but data-driven decisions tend to produce \
better outcomes.\n\n\
What specific aspect would you like me to dig deeper into?".into()
            } else {
                format!(
                    "Thank you for bringing this up. Regarding \"{}\", here's my \
strategic perspective:\n\n\
The landscape around this topic is shifting, and those who position themselves \
well now will have a significant advantage. Here's what I'd focus on:\n\n\
**Immediate Priority:** Assess your current position honestly. Where are your \
strengths? Where are the gaps? Understanding your starting point is crucial for \
charting the right course.\n\n\
**Strategic Consideration:** Don't try to do everything at once. The most \
successful people I advise are those who identify the single highest-leverage \
action and execute it excellently, rather than spreading themselves thin across \
multiple initiatives.\n\n\
**Watch Out For:** Confirmation bias. It's human nature to seek out information \
that supports what we already believe. Actively seek out perspectives that \
challenge your assumptions.\n\n\
I'm here to help you think through the specifics. What's the most pressing \
aspect of this for you right now?",
                    if message.len() > 60 { &message[..60] } else { message }
                )
            }
        }

        "translator" => {
            if low.contains("translate") || low.contains("spanish") || low.contains("french")
                || low.contains("german") || low.contains("japanese") || low.contains("chinese")
            {
                "I can help with that translation! Here's what I can offer:\n\n\
**Translation Notes:**\n\
When translating, context matters enormously. A direct word-for-word translation \
often misses the nuance, tone, and cultural context of the original.\n\n\
For your request, I've considered:\n\
- **Register** - Is this formal or informal? Written or spoken?\n\
- **Audience** - Who will read this? Native speakers or learners?\n\
- **Purpose** - Is accuracy or naturalness more important?\n\n\
**My Approach:**\n\
I prioritize natural-sounding translations that preserve the intent and \
emotion of the original, rather than producing awkward literal translations. \
If there are culturally specific references, I'll adapt them appropriately \
and note any significant deviations from the source.\n\n\
Please share the specific text you'd like translated, along with the target \
language, and I'll provide both the translation and any relevant notes about \
word choices or cultural considerations.".into()
            } else {
                format!(
                    "I see you've sent: \"{}\"\n\n\
As a translator, I can work with over 30 languages. Let me share how I can \
help:\n\n\
**Available Services:**\n\
- Direct translation between any supported language pair\n\
- Localization (adapting content for a specific culture/region)\n\
- Proofreading of existing translations\n\
- Explanation of idiomatic expressions and their equivalents\n\
- Grammar and usage notes for language learners\n\n\
**Supported Languages Include:**\n\
English, Spanish, French, German, Italian, Portuguese, Chinese (Simplified \
& Traditional), Japanese, Korean, Arabic, Russian, Hindi, Dutch, Swedish, \
Polish, Turkish, Vietnamese, Thai, and more.\n\n\
To get started, just tell me:\n\
1. The text you want translated\n\
2. The source language (or I can detect it)\n\
3. The target language\n\
4. Any context about tone or audience\n\n\
I'll handle the rest!",
                    if message.len() > 50 { &message[..50] } else { message }
                )
            }
        }

        // Default persona
        _ => {
            format!(
                "Thank you for your message. I've received your input regarding \
\"{topic}\" and I'm ready to help.\n\n\
As your AI assistant, I can adapt to whatever you need. Whether you're looking \
for information, creative help, problem-solving, or just a conversation, I'm \
here for it.\n\n\
Based on what you've shared, here are a few directions we could go:\n\
- I can provide a detailed analysis or explanation\n\
- I can help you brainstorm ideas and solutions\n\
- I can create content, code, or documentation\n\
- I can offer feedback on your existing work\n\n\
What would be most helpful for you right now?",
                topic = if message.len() > 60 { &message[..60] } else { message }
            )
        }
    }
}

fn stream_from_text(
    text: String,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    let chunks: Vec<String> = text
        .split_whitespace()
        .map(|w| format!("{w} "))
        .collect();

    Box::pin(futures::stream::iter(
        chunks
            .into_iter()
            .map(|chunk| Ok(axum::response::sse::Event::default().data(chunk))),
    ))
}

pub fn mock_chat_stream(
    message: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_chat_response(message))
}

pub fn mock_content_stream(
    platform: &str,
    tone: &str,
    prompt: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_content(platform, tone, prompt))
}

pub fn mock_code_stream(
    language: &str,
    description: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_code(language, description))
}

pub fn mock_email_stream(
    email_type: &str,
    subject: &str,
    tone: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_email(email_type, subject, tone))
}

pub fn mock_resume_stream(
    name: &str,
    experience: &str,
    skills: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_resume(name, experience, skills))
}

pub fn mock_bot_stream(
    persona: &str,
    message: &str,
) -> futures::stream::BoxStream<
    'static,
    Result<axum::response::sse::Event, std::convert::Infallible>,
> {
    stream_from_text(mock_bot_response(persona, message))
}

pub fn mock_image_bytes() -> Vec<u8> {
    let data_url = mock_image_base64();
    let b64 = data_url.split(',').nth(1).unwrap_or(data_url.as_str());
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .unwrap_or_default()
}
