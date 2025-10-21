use rand::Rng;

pub struct Quotes;

impl Quotes {
    pub const MOTIVATIONAL_QUOTES: [&'static str; 50] = [
        "💅 Slay the day, bestie! Your tasks don't stand a chance.",
        "✨ Main character energy activated! Let's get it done.",
        "🔥 You're doing amazing, sweetie! Keep grinding.",
        "💯 No cap, you're crushing it today!",
        "🚀 Living rent-free in productivity's mind. Period.",
        "⚡ That's hot. That's a hot way to handle tasks.",
        "🎯 On fleek and on track! Let's gooo!",
        "💪 Built different. Your tasks could never.",
        "🌟 Understood the assignment and ate! No crumbs left.",
        "😎 CEO of getting things done. Respectfully.",
        "🦋 Glow up szn! These tasks are going down.",
        "💖 Self-care is completing your to-do list, babes.",
        "🎨 Creating your reality one task at a time. Iconic.",
        "🌈 Manifest that productivity, queen/king!",
        "👑 Royal vibes only. Peasant tasks, bow down.",
        "🍵 Spill the tea? More like spill the completed tasks.",
        "💸 Investing in yourself by investing in tasks. Smart.",
        "🎪 The circus is empty because you're handling it.",
        "🧠 Big brain energy. These tasks never saw it coming.",
        "✌️ Peace out, procrastination. We don't know her.",
        "🎭 Acting like you can't do it? Oscar-worthy. Now do it.",
        "🌸 Soft life but hard work. That's the balance.",
        "🎮 Leveling up IRL. Achievement unlocked!",
        "🌊 Riding the productivity wave. Surf's up!",
        "🎵 That task list? It's giving completed.",
        "🦄 Rare, magical, and getting stuff done. That's you.",
        "🍀 Lucky? Nah, just that good at productivity.",
        "🎬 Lights, camera, action! Your moment is now.",
        "🔮 The future is bright and task-free. Keep going.",
        "🎯 Hitting different when you're this productive.",
        "💎 Pressure makes diamonds. You're shining, boo.",
        "🌙 Night or day, you slay. No days off from greatness.",
        "🎪 Not the clown, but the whole circus? Nah, the ringmaster.",
        "🧃 Sipping on that productivity juice. Refreshing!",
        "🎨 Canvas of life? You're painting masterpieces.",
        "🌻 Bloom where you're planted. And you're flourishing!",
        "💫 Star quality. Your tasks are starstruck.",
        "🎸 Rock on! Those tasks don't know what hit them.",
        "🏆 Championship mindset. Trophy incoming.",
        "🍕 Delivering results faster than pizza. Hot and fresh.",
        "🎁 Gift yourself the satisfaction of done tasks.",
        "🌍 World domination starts with task completion.",
        "🎊 Every task completed is a mini celebration.",
        "🦾 Stronger than your excuses. Prove it.",
        "🎯 Bullseye! Direct hit on productivity.",
        "🌺 Thriving, not just surviving. Look at you go!",
        "🎪 Your show, your rules. Make it legendary.",
        "🍒 Cherry on top? Completing this task. Sweet!",
        "⭐ Five-star performance. Michelin-level productivity.",
        "🎨 Picasso could never paint tasks this beautifully done.",
    ];

    pub fn get_random() -> &'static str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..Self::MOTIVATIONAL_QUOTES.len());
        Self::MOTIVATIONAL_QUOTES[index]
    }
}

