package scraper_test

import (
	"fmt"
	"github.com/PuerkitoBio/goquery"
	"golang.org/x/exp/maps"
	"golang.org/x/exp/slices"
	"metal-releases/internal/models"
	"metal-releases/internal/scraper"
	"os"
	"path"
	"runtime"
	"testing"
	"time"
)

func TestScrapeMetalReleases(t *testing.T) {
	want := &models.Calendar{
		January: models.Releases{
			7: {
				{Artist: "Atrocity", Album: "Unspoken Names (Demo 1991) (EP)"},
				{Artist: "Infected Rain", Album: "Ecdysis"},
				{Artist: "Schwarzer Engel", Album: "Sieben"},
			},
			14: {
				{Artist: "Enterprise Earth", Album: "The Chosen"},
				{Artist: "Fit for an Autopsy", Album: "Oh What the Future Holds"},
				{Artist: "Ilium", Album: "Quantum Evolution Event (EP)"},
				{Artist: "Shadow of Intent", Album: "Elegy"},
				{Artist: "Skillet", Album: "Dominion"},
				{Artist: "Tony Martin", Album: "Thorns"},
				{Artist: "Underoath", Album: "Voyeurist"},
			},
			21: {
				{Artist: "Ashes of Ares", Album: "Emperors and Fools"},
				{Artist: "Asking Alexandria", Album: "Never Gonna Learn (EP)"},
				{Artist: "Battle Beast", Album: "Circus of Doom"},
				{Artist: "Boris", Album: "W"},
				{Artist: "Confess", Album: "Revenge at All Costs"},
				{Artist: "Giant", Album: "Shifting Time"},
				{Artist: "Iced Earth", Album: "A Narrative Soundscape"},
				{Artist: "Kissin' Dynamite", Album: "Not the End of the Road"},
				{Artist: "Sonata Arctica", Album: "Acoustic Adventures – Volume One"},
				{Artist: "Tokyo Blade", Album: "Fury"},
			},
			28: {
				{Artist: "Celeste", Album: "Assassine(s)"},
				{Artist: "Cloakroom", Album: "Dissolution Wave"},
				{Artist: "Dawn of Solace", Album: "Flames of Perdition"},
				{Artist: "Emerald Sun", Album: "Kingdom of Gods"},
				{Artist: "Krallice", Album: "Crystalline Exhaustion"},
				{Artist: "Lana Lane", Album: "Neptune Blue"},
				{Artist: "The Last Ten Seconds of Life", Album: "The Last Ten Seconds of Life"},
				{Artist: "Lawnmower Deth", Album: "Blunt Cutters"},
				{Artist: "Praying Mantis", Album: "Katharsis"},
				{Artist: "The Quill", Album: "Live, New, Borrowed, Blue (compilation album)"},
				{Artist: "Steve Vai", Album: "Inviolate"},
			},
		},
		February: models.Releases{
			4: {
				{Artist: "Abysmal Dawn", Album: "Nightmare Frontier (EP)"},
				{Artist: "Bevar Sea", Album: "The Timeless Zone"},
				{Artist: "Hed PE", Album: "Califas Worldwide"},
				{Artist: "Korn", Album: "Requiem"},
				{Artist: "Mystic Circle", Album: "Mystic Circle"},
				{Artist: "Persefone", Album: "Metanoia"},
				{Artist: "Rolo Tomassi", Album: "Where Myth Becomes Memory"},
				{Artist: "Saxon", Album: "Carpe Diem"},
				{Artist: "Venom Prison", Album: "Erebos"},
			},
			11: {
				{Artist: "Amorphis", Album: "Halo"},
				{Artist: "Author & Punisher", Album: "Krüller"},
				{Artist: "Cult of Luna", Album: "The Long Road North"},
				{Artist: "Girish and The Chronicles", Album: "Hail to the Heroes"},
				{Artist: "Napalm Death", Album: "Resentment Is Always Seismic – A Final Throw of Throes (mini-album)"},
				{Artist: "Once Human", Album: "Scar Weaver"},
				{Artist: "The Silent Wedding", Album: "Ego Path"},
				{Artist: "Slash feat. Myles Kennedy & the Conspirators", Album: "4"},
				{Artist: "Tersivel", Album: "To the Orphic Void"},
				{Artist: "Voivod", Album: "Synchro Anarchy"},
				{Artist: "Zeal & Ardor", Album: "Zeal & Ardor"},
			},
			18: {
				{Artist: "Annihilator", Album: "Metal II"},
				{Artist: "Bloodywood", Album: "Rakshak"},
				{Artist: "Dagoba", Album: "By Night"},
				{Artist: "Esprit D'Air", Album: "Oceans"},
				{Artist: "Immolation", Album: "Acts of God"},
				{Artist: "Matt Pike", Album: "Pike vs. the Automaton"},
				{Artist: "Nightrage", Album: "Abyss Rising"},
				{Artist: "Spirits of Fire", Album: "Embrace the Unknown"},
				{Artist: "Star One", Album: "Revel in Time"},
			},
			25: {
				{Artist: "Allegaeon", Album: "Damnum"},
				{Artist: "Bad Omens", Album: "The Death of Peace of Mind"},
				{Artist: "Blood Incantation", Album: "Timewave Zero"},
				{Artist: "Corey Taylor", Album: "CMFB ...Sides (covers album)"},
				{Artist: "Diablo", Album: "When All the Rivers Are Silent"},
				{Artist: "Eight Bells", Album: "Legacy of Ruin"},
				{Artist: `George "Corpsegrinder" Fisher`, Album: "Corpsegrinder"},
				{Artist: "Guns N' Roses", Album: "Hard Skool (EP)"},
				{Artist: "HammerFall", Album: "Hammer of Dawn"},
				{Artist: "Metalucifer", Album: "Heavy Metal Ninja (mini-album)"},
				{Artist: "Scorpions", Album: "Rock Believer"},
				{Artist: "Shape of Despair", Album: "Return to the Void"},
				{Artist: "Svartsot", Album: "Kumbl"},
				{Artist: "Tygers of Pan Tang", Album: "A New Heartbeat (EP)"},
			},
		},
		March: models.Releases{
			4: {
				{Artist: "10 Years", Album: "Deconstructed"},
				{Artist: "Crowbar", Album: "Zero and Below"},
				{Artist: "Eric Wagner", Album: "In the Lonely Light of Mourning"},
				{Artist: "Flaw", Album: "Revival (covers album)"},
				{Artist: "Sabaton", Album: "The War to End All Wars"},
				{Artist: "Sunflower Dead", Album: "March of the Leper"},
				{Artist: "Ty Tabor", Album: "Shades"},
				{Artist: "Vein.fm", Album: "This World Is Going to Ruin You"},
				{Artist: "Vio-lence", Album: "Let the World Burn (EP)"},
				{Artist: "Warrior Soul", Album: "Out on Bail"},
			},
			5: {
				{Artist: "King Gizzard & the Lizard Wizard", Album: "Made in Timeland"},
				{Artist: "Troglodyte", Album: "The Hierarchical Ecological Succession: Welcome to the Food Chain"},
			},
			11: {
				{Artist: "Black Pantera", Album: "Ascensão"},
				{Artist: "Brandon Boyd", Album: "Echoes and Cocoons"},
				{Artist: "Claustrofobia", Album: "Unleeched"},
				{Artist: "Cloven Hoof", Album: "Time Assassin"},
				{Artist: "Ghost", Album: "Impera"},
				{Artist: "Grim Reaper", Album: "Reaping the Whirlwind (live album)"},
				{Artist: "Kiss", Album: "Off the Soundboard: Live in Virginia Beach (live album)"},
				{Artist: "Love/Hate", Album: "HELL, CA"},
				{Artist: "New Horizon", Album: "Gate of the Gods"},
				{Artist: "Shaman's Harvest", Album: "Rebelator"},
				{Artist: "Wolves at the Gate", Album: "Eulogies"},
			},
			12: {
				{Artist: "Dog Fashion Disco", Album: "Cult Classic"},
			},
			18: {
				{Artist: "Agathodaimon", Album: "The Seven"},
				{Artist: "Dark Funeral", Album: "We Are the Apocalypse"},
				{Artist: "Dawn of Ashes", Album: "Scars of the Broken"},
				{Artist: "Manigance", Album: "Le bal des ombres"},
				{Artist: "Ronni Le Tekrø", Album: "Bigfoot TV"},
				{Artist: "Ronnie Atkins", Album: "Make It Count"},
				{Artist: "Stabbing Westward", Album: "Chasing Ghosts"},
				{Artist: "Týr", Album: "A Night at the Nordic House (live album)"},
			},
			23: {
				{Artist: "Deathspell Omega", Album: "The Long Defeat"},
			},
			25: {
				{Artist: "Abbath", Album: "Dread Reaver"},
				{Artist: "Animals as Leaders", Album: "Parrhesia"},
				{Artist: "Architects", Album: "For Those That Wish to Exist at Abbey Road (live album)"},
				{Artist: "BillyBio", Album: "Leaders and Liars"},
				{Artist: "Crystal Viper", Album: "The Last Axeman (mini-album)"},
				{Artist: "Eucharist", Album: "I Am the Void"},
				{Artist: "Hardcore Superstar", Album: "Abrakadabra"},
				{Artist: "Killing Joke", Album: "Lord of Chaos (EP)"},
				{Artist: "Michael Romeo", Album: "War of the Worlds, Pt. 2"},
				{Artist: "Pist.On", Album: "Cold World EP (EP)"},
				{Artist: "Reckless Love", Album: "Turborider"},
			},
		},
		April: models.Releases{
			1: {
				{Artist: "Centinex", Album: "The Pestilence (EP)"},
				{Artist: "Kublai Khan", Album: "Lowest Form of Animal (EP)"},
				{Artist: "Lords of the Trident", Album: "The Offering"},
				{Artist: "Meshuggah", Album: "Immutable"},
				{Artist: "Nekrogoblikon", Album: "The Fundamental Slimes and Humours"},
				{Artist: "Satan", Album: "Earth Infernal"},
				{Artist: "Trick or Treat", Album: "Creepy Symphonies"},
				{Artist: "Wolf", Album: "Shadowland"},
			},
			8: {
				{Artist: "Destruction", Album: "Diabolical"},
				{Artist: "Hällas", Album: "Isle of Wisdom"},
				{Artist: "Incite", Album: "Wake Up Dead"},
				{Artist: "Inglorious", Album: "MMXXI Live at the Phoenix (live album)"},
				{Artist: "Mors Principium Est", Album: "Liberate the Unborn Inhumanity (compilation album)"},
				{Artist: "Papa Roach", Album: "Ego Trip"},
				{Artist: "Terzij de Horde", Album: "In One of These, I Am Your Enemy"},
				{Artist: "Treat", Album: "The Endgame"},
			},
			14: {
				{Artist: "Psychostick", Album: "... and Stuff (compilation album)"},
			},
			15: {
				{Artist: "Abated Mass of Flesh", Album: "The Existence of Human Suffering"},
				{Artist: "Axel Rudi Pell", Album: "Lost XXIII"},
				{Artist: "Cancer Bats", Album: "Psychic Jailbreak"},
				{Artist: "Grand Belial's Key", Album: "Kohanic Charmers"},
				{Artist: "JBO", Album: "Planet Pink"},
				{Artist: "Månegarm", Album: "Ynglingaättens Öde"},
				{Artist: "Monuments", Album: "In Stasis"},
				{Artist: "Nazareth", Album: "Surviving the Law"},
				{Artist: "Powerglove", Album: "Flawless Victory (EP)"},
				{Artist: "Ronnie Romero", Album: "Raised on Radio (covers album)"},
				{Artist: "Semblant", Album: "Vermilion Eclipse"},
				{Artist: "These Arms Are Snakes", Album: "Duct Tape & Shivering Crows (compilation album)"},
			},
			22: {
				{Artist: "Archgoat", Album: "All Christianity Ends (EP)"},
				{Artist: "Caliban", Album: "Dystopia"},
				{Artist: "Die Apokalyptischen Reiter", Album: "Wilde Kinder"},
				{Artist: "King Gizzard & the Lizard Wizard", Album: "Omnium Gatherum"},
				{Artist: "Märvel", Album: "Graces Came with Malice"},
				{Artist: "Miseration", Album: "Black Miracles and Dark Wonders"},
				{Artist: "Northlane", Album: "Obsidian"},
				{Artist: "Ocean Grove", Album: "Up in the Air Forever"},
				{Artist: "Primus", Album: "Conspiranoid (EP)"},
				{Artist: "Skull Fist", Album: "Paid in Full"},
				{Artist: "Somali Yacht Club", Album: "The Space"},
				{Artist: "Speckmann Project 	Fiends of Emptiness"},
				{Artist: "Udo Dirkschneider", Album: "My Way (covers album)"},
			},
			23: {
				{Artist: "Charlie Benante", Album: "Moving Pitchers (EP)"},
				{Artist: "Kirk Hammett", Album: "Portals (EP)"},
				{Artist: "The Lord", Album: "Forest Nocturne"},
			},
			29: {
				{Artist: "Al-Namrood", Album: "Worship the Degenerate"},
				{Artist: "Crashdïet", Album: "Automaton"},
				{Artist: "The Gathering", Album: "Beautiful Distortion"},
				{Artist: "Helms Alee", Album: "Keep This Be the Way"},
				{Artist: "Rammstein", Album: "Zeit"},
				{Artist: "Thunder", Album: "Dopamine"},
				{Artist: "Void of Vision", Album: "Chronicles II: Heaven (EP)"},
				{Artist: "Vulcano", Album: "Stone Orange"},
				{Artist: "Watain", Album: "The Agony & Ecstasy of Watain"},
			},
		},
		May: models.Releases{
			6: {
				{Artist: "Depressed Mode", Album: "Decade of Silence"},
				{Artist: "Fozzy", Album: "Boombox"},
				{Artist: "Halestorm", Album: "Back from the Dead"},
				{Artist: "Ibaraki", Album: "Rashomon"},
				{Artist: "Jani Liimatainen", Album: "My Father's Son"},
				{Artist: "Jeff Scott Soto", Album: "Complicated"},
				{Artist: "Lord of the Lost", Album: "The Heartbeat of the Devil (EP)"},
				{Artist: "Puppy", Album: "Pure Evil"},
				{Artist: "Three Days Grace", Album: "Explosions"},
				{Artist: "Ufomammut", Album: "Fenice"},
				{Artist: "Upon a Burning Body", Album: "Fury"},
				{Artist: "Windwaker", Album: "Love Language"},
			},
			13: {
				{Artist: "Demiricous", Album: "III: Chaotic Lethal"},
				{Artist: "Graham Bonnet Band", Album: "Day Out in Nowhere"},
				{Artist: "Jungle Rot", Album: "A Call to Arms"},
				{Artist: "Misery Index", Album: "Complete Control"},
				{Artist: "Primitive Man", Album: "Insurmountable (EP)"},
				{Artist: "Visions of Atlantis", Album: "Pirates"},
				{Artist: "Zero Hour", Album: "Agenda 21"},
			},
			18: {
				{Artist: "Novelbright", Album: "Assort"},
			},
			20: {
				{Artist: "Anvil", Album: "Impact Is Imminent"},
				{Artist: "Blut Aus Nord", Album: "Disharmonium – Undreamable Abysses"},
				{Artist: "Cave In", Album: "Heavy Pendulum"},
				{Artist: "Chuck Wright's Sheltering Sky", Album: "Chuck Wright's Sheltering Sky"},
				{Artist: "Evergrey", Album: "A Heartless Portrait (The Orphean Testament)"},
				{Artist: "James LaBrie", Album: "Beautiful Shade of Gray"},
				{Artist: "Malevolence", Album: "Malicious Intent"},
				{Artist: "Ratos de Porão", Album: "Necropolítica"},
				{Artist: "Sadist", Album: "Firescorched"},
				{Artist: "Septicflesh", Album: "Modern Primitive"},
				{Artist: "Spheric Universe Experience", Album: "Back Home"},
				{Artist: "Zinny Zan", Album: "Lullabies for the Masses"},
			},
			25: {
				{Artist: "Man with a Mission", Album: "Break and Cross the Walls II"},
			},
			27: {
				{Artist: "Baest", Album: "Justitia (EP)"},
				{Artist: "Brutality", Album: "Sempiternity"},
				{Artist: "Cadaveria", Album: "Emptiness"},
				{Artist: "Crematory", Album: "Inglorious Darkness"},
				{Artist: "Decapitated", Album: "Cancer Culture"},
				{Artist: "Def Leppard", Album: "Diamond Star Halos"},
				{Artist: "Holocausto Canibal", Album: "Crueza Ferina"},
				{Artist: "Lord Belial", Album: "Rapture"},
				{Artist: "Michael Schenker Group", Album: "Universal"},
				{Artist: "Mournful Congregation", Album: "The Exuviae of Gods – Part I (EP)"},
				{Artist: "Odd Crew", Album: "Dark Matters (Part 1)"},
				{Artist: "Trollfest", Album: "Flamingo Overlord"},
			},
			31: {
				{Artist: "Ribspreader", Album: "Crypt World"},
			},
		},
		June: models.Releases{
			3: {
				{Artist: "The Algorithm", Album: "Data Renaissance"},
				{Artist: "Astronoid", Album: "Radiant Bloom"},
				{Artist: "Battlelore", Album: "The Return of the Shadow"},
				{Artist: "Bleed from Within", Album: "Shrine"},
				{Artist: "Gwar", Album: "The New Dark Ages"},
				{Artist: "Killswitch Engage", Album: "Live at the Palladium (live album)"},
				{Artist: "Las Cruces", Album: "Cosmic Tears"},
				{Artist: "Memphis May Fire", Album: "Remade in Misery"},
				{Artist: "Origin", Album: "Chaosmos"},
				{Artist: "Red Handed Denial", Album: "I'd Rather Be Asleep"},
				{Artist: "Thornhill", Album: "Heroine"},
			},
			5: {
				{Artist: "Wolfsbane", Album: "Genius"},
			},
			10: {
				{Artist: "Billy Howerdel", Album: "What Normal Was"},
				{Artist: "Deadguy", Album: "Buyer's Remorse: Live from the Decibel Magazine Metal & Beer Fest (live album)"},
				{Artist: "downset.", Album: "Maintain"},
				{Artist: "Dragged Under", Album: "Upright Animals"},
				{Artist: "Kiss", Album: "Off the Soundboard: Live at Donington 1996 (live album)"},
				{Artist: "Kreator", Album: "Hate Über Alles"},
				{Artist: "Michael Monroe", Album: "I Live Too Fast to Die Young"},
				{Artist: "Motionless in White", Album: "Scoring the End of the World"},
				{Artist: "Satyricon", Album: "Satyricon & Munch"},
				{Artist: "Schandmaul", Album: "Knüppel aus dem Sack"},
				{Artist: "Secrets", Album: "The Collapse"},
				{Artist: "Seventh Wonder", Album: "The Testament"},
				{Artist: "Severe Torture", Album: "Fisting the Sockets (EP)"},
				{Artist: "Soreption", Album: "Jord"},
				{Artist: "Tierra Santa", Album: "Destino"},
				{Artist: "William DuVall", Album: "11.12.21 Live-In-Studio Nashville"},
				{Artist: "Wind Rose", Album: "Warfront"},
			},
			13: {
				{Artist: "Tombs", Album: "Ex Oblivion (EP)"},
			},
			15: {
				{Artist: "Dir En Grey", Album: "Phalaris"},
				{Artist: "Rings of Saturn", Album: "Rings of Saturn"},
			},
			17: {
				{Artist: "Civil War", Album: "Invaders"},
				{Artist: "Infanteria", Album: "Patriarch"},
				{Artist: "Jorn", Album: "Over the Horizon Radar"},
				{Artist: "Oni", Album: "Loathing Light"},
				{Artist: "Seven Kingdoms", Album: "Zenith"},
				{Artist: "Tungsten", Album: "Bliss"},
			},
			22: {
				{Artist: "Manowar", Album: "The Revenge of Odysseus (Highlights) (EP)"},
				{Artist: "Spiritbox", Album: "Rotoscope (EP)"},
			},
			24: {
				{Artist: "Alestorm", Album: "Seventh Rum of a Seventh Rum"},
				{Artist: "Betraying the Martyrs", Album: "Silver Lining (EP)"},
				{Artist: "Between the Buried and Me", Album: "The Great Misdirect Live (live album)"},
				{Artist: "Black River", Album: "Generation aXe"},
				{Artist: "Black Stone Cherry", Album: "Live from the Royal Albert Hall... Y'All (live album)"},
				{Artist: "Coheed and Cambria", Album: "Vaxis – Act II: A Window of the Waking Mind"},
				{Artist: "Darkane", Album: "Inhuman Spirits"},
				{Artist: "Dawn of Destiny", Album: "Of Silence"},
				{Artist: "Enphin", Album: "End Cut"},
				{Artist: "Khold", Album: "Svartsyn"},
				{Artist: "Paganizer", Album: "Beyond the Macabre"},
				{Artist: "Porcupine Tree", Album: "Closure/Continuation"},
				{Artist: "Projected", Album: "Hypoxia"},
				{Artist: "Victorius", Album: "Dinosaur Warfare Pt.2 – The Great Ninja War"},
			},
			30: {
				{Artist: "Bleeding Through 	Rage (EP)"},
			},
		},
		July: models.Releases{
			1: {
				{Artist: "Derek Sherinian", Album: "Vortex[377]"},
				{Artist: "Greg Puciato", Album: "Mirrorcell[378]"},
				{Artist: "Haunt", Album: "Windows of Your Heart[379]"},
				{Artist: "Holy Dragons", Album: "Jörmungandr – The Serpent of the World[380]"},
				{Artist: "Massacre", Album: "Mythos (EP)[381]"},
				{Artist: "Municipal Waste", Album: "Electrified Brain[382]"},
				{Artist: "Randy Holden", Album: "Population III[383]"},
				{Artist: "Saint Asonia", Album: "Introvert (EP)[384]"},
				{Artist: "Shinedown", Album: "Planet Zero[385]"},
				{Artist: "Superheist", Album: "MMXX[386]"},
			},
			6: {
				{Artist: "Coldrain", Album: "Nonnegative"},
			},
			8: {
				{Artist: "Altaria", Album: "Wisdom"},
				{Artist: "Blind Channel", Album: "Lifestyles of the Sick & Dangerous"},
				{Artist: "Powerwolf", Album: "The Monumental Mass – A Cinematic Metal Event (live album)"},
				{Artist: "Wormrot", Album: "Hiss"},
			},
			13: {
				{Artist: "Obituary", Album: "Cause of Death – Live Infection (live album)"},
				{Artist: "Obituary", Album: "Slowly We Rot – Live & Rotting (live album)"},
			},
			15: {
				{Artist: "Antigama", Album: "Whiteout"},
				{Artist: "Jack Starr's Burning Starr", Album: "Souls of the Innocent"},
				{Artist: "Mantar", Album: "Pain Is Forever and This Is the End"},
				{Artist: "Senses Fail", Album: "Hell Is in Your Head"},
				{Artist: "Sinner", Album: "Brotherhood"},
			},
			22: {
				{Artist: "Hatriot", Album: "The Vale of Shadows"},
				{Artist: "Imperial Triumphant", Album: "Spirit of Ecstasy"},
				{Artist: "Karl Sanders", Album: "Saurian Apocalypse"},
				{Artist: "Oceans of Slumber", Album: "Starlight and Ash"},
				{Artist: "Palisades", Album: "Reaching Hypercritical"},
				{Artist: "Scar for Life", Album: "Sociophobia"},
				{Artist: "Witchery", Album: "Nightside"},
			},
			28: {
				{Artist: "Bad Wolves", Album: "Sacred Kiss (EP)"},
				{Artist: "Incantation", Album: "Tricennial of Blasphemy (compilation album)"},
			},
			29: {
				{Artist: "Belphegor", Album: "The Devils"},
				{Artist: "Black Magnet", Album: "Body Prophecy"},
				{Artist: "Chat Pile", Album: "God's Country"},
				{Artist: "Krisiun", Album: "Mortem Solis"},
				{Artist: "Stick to Your Guns", Album: "Spectre"},
				{Artist: "Torture Killer", Album: "Dead Inside (EP)"},
			},
		},
		August: models.Releases{
			4: {
				{Artist: "Tom Hunting", Album: "Hunting Party (EP)"},
			},
			5: {
				{Artist: "Abaddon Incarnate", Album: "The Wretched Sermon"},
				{Artist: "Amon Amarth", Album: "The Great Heathen Army"},
				{Artist: "Dub War", Album: "Westgate Under Fire"},
				{Artist: "Einherjer", Album: "Norse and Dangerous (Live... from the Land of Legends) (live album)"},
				{Artist: "H.E.A.T", Album: "Force Majeure"},
				{Artist: "Psycroptic", Album: "Divine Council"},
				{Artist: "Soulfly", Album: "Totem"},
				{Artist: "Toxik", Album: "Dis Morta"},
				{Artist: "Vanden Plas", Album: "Live & Immortal (live album)"},
			},
			12: {
				{Artist: "Arch Enemy", Album: "Deceivers"},
				{Artist: "Boris", Album: "Heavy Rocks"},
				{Artist: "The Halo Effect", Album: "Days of the Lost"},
				{Artist: "Hollywood Undead", Album: "Hotel Kalifornia"},
				{Artist: "Jackyl", Album: "30 Coming in Hot (compilation album)"},
				{Artist: "Locrian", Album: "New Catastrophism"},
				{Artist: "Ghost", Album: "Frontiers (EP)"},
				{Artist: "Norma Jean", Album: "Deathrattle Sing for Me"},
				{Artist: "Wolfbrigade", Album: "Anti-Tank Dogs (EP)"},
			},
			14: {
				{Artist: "Melvins", Album: "Bad Mood Rising"},
			},
			19: {
				{Artist: "Conan", Album: "Evidence of Immortality"},
				{Artist: "Five Finger Death Punch", Album: "AfterLife"},
				{Artist: "Heilung", Album: "Drif"},
				{Artist: "I Prevail", Album: "True Power"},
				{Artist: "Lillian Axe", Album: "From Womb to Tomb"},
				{Artist: "Parasite Inc.", Album: "Cyan Night Dreams"},
				{Artist: "Psyclon Nine", Album: "Less to Heaven"},
				{Artist: "Russian Circles", Album: "Gnosis"},
				{Artist: "Soilwork", Album: "Övergivenheten"},
				{Artist: "Spirit Adrift", Album: "20 Centuries Gone (compilation album)"},
			},
			26: {
				{Artist: "Becoming the Archetype", Album: "Children of the Great Extinction"},
				{Artist: "Brymir", Album: "Voices in the Sky"},
				{Artist: "Dynazty", Album: "Final Advent"},
				{Artist: "Grave Digger", Album: "Symbol of Eternity"},
				{Artist: "Lacrimas Profundere", Album: "How to Shroud Yourself with Night"},
				{Artist: "Long Distance Calling", Album: "Eraser"},
				{Artist: "Machine Head", Album: "Of Kingdom and Crown"},
				{Artist: "Santa Cruz", Album: "The Return of the Kings"},
				{Artist: "Sigh", Album: "Shiki"},
				{Artist: "Soil", Album: "Play It Forward (covers album)"},
				{Artist: "Tad Morose", Album: "March of the Obsequious"},
			},
			27: {
				{Artist: "Imperial Age", Album: "New World"},
			},
		},
		September: models.Releases{
			1: {
				{Artist: "Oceans Ate Alaska", Album: "Disparity"},
			},
			2: {
				{Artist: "Blind Guardian", Album: "The God Machine"},
				{Artist: "The Callous Daoboys", Album: "Celebrity Therapist"},
				{Artist: "The Hu", Album: "Rumble of Thunder"},
				{Artist: "Mad Max", Album: "Wings of Time"},
				{Artist: "Mantic Ritual", Album: "Heart Set Stone (EP)"},
				{Artist: "King's X", Album: "Three Sides of One"},
				{Artist: "Megadeth", Album: "The Sick, the Dying... and the Dead!"},
				{Artist: "Mike Tramp", Album: "For Første Gang"},
				{Artist: "Miss May I", Album: "Curse of Existence"},
				{Artist: "Novelists", Album: "Déjà Vu"},
			},
			9: {
				{Artist: "Allen/Olzon", Album: "Army of Dreamers"},
				{Artist: "Bloodbath", Album: "Survival of the Sickest"},
				{Artist: "Fallujah", Album: "Empyrean"},
				{Artist: "Holy Fawn", Album: "Dimensional Bleed"},
				{Artist: "Kiss", Album: "Off the Soundboard: Live in Des Moines 1977 (live album)"},
				{Artist: "KMFDM", Album: "Hyëna"},
				{Artist: "Mezarkabul", Album: "Makina Elektrika"},
				{Artist: "Ozzy Osbourne", Album: "Patient Number 9"},
				{Artist: "Parkway Drive", Album: "Darker Still"},
				{Artist: "Revocation", Album: "Netherheaven"},
				{Artist: "Stray from the Path", Album: "Euthanasia"},
				{Artist: "Trauma", Album: "Awakening"},
				{Artist: "Ville Laihiala & Saattajat", Album: "Ei Meillä Ole Kuin Loisemme"},
			},
			16: {
				{Artist: "The 69 Eyes", Album: "Drive (EP)"},
				{Artist: "Behemoth", Album: "Opvs Contra Natvram"},
				{Artist: "Clutch", Album: "Sunrise on Slaughter Beach"},
				{Artist: "Destrage", Album: "SO MUCH.too much."},
				{Artist: "The Devil Wears Prada", Album: "Color Decay"},
				{Artist: "Edenbridge", Album: "Shangri-La"},
				{Artist: "Electric Callboy", Album: "Tekkno"},
				{Artist: "Epoch of Unlight", Album: "At War with the Multiverse"},
				{Artist: "Hartmann", Album: "Get Over It"},
				{Artist: "Hetroertzen", Album: "Phosphorus, Vol.1"},
				{Artist: "House of Lords", Album: "Saints and Sinners"},
				{Artist: "Marco Mendoza", Album: "New Direction"},
				{Artist: "Omega Diatribe", Album: "My Sphere (EP)"},
				{Artist: "Spiritus Mortis", Album: "The Great Seal"},
			},
			23: {
				{Artist: "KEN mode", Album: "Null"},
				{Artist: "Moonspell", Album: "From Down Below – Live 80 Meters Deep (live album)"},
				{Artist: "OvO", Album: "Ignoto"},
				{Artist: "Razor", Album: "Cycle of Contempt"},
				{Artist: "Silent Knight", Album: "Full Force"},
				{Artist: "Stratovarius", Album: "Survive"},
				{Artist: "Venom Inc.", Album: "There's Only Black"},
			},
			30: {
				{Artist: "Autopsy", Album: "Morbidity Triumphant"},
				{Artist: "Drowning Pool", Album: "Strike a Nerve"},
				{Artist: "Rage", Album: "Spreading the Plague (EP)"},
				{Artist: "Raven", Album: "Leave 'Em Bleeding (compilation album)"},
				{Artist: "Sammy Hagar and the Circle", Album: "Crazy Times"},
				{Artist: "Sceptic", Album: "Nailed to Ignorance"},
				{Artist: "Slipknot", Album: "The End, So Far"},
				{Artist: "Sonata Arctica", Album: "Acoustic Adventures – Volume Two"},
				{Artist: "Tankard", Album: "Pavlov's Dawgs"},
			},
		},
		October: models.Releases{
			1: {
				{Artist: "Acid Witch", Album: "Rot Among Us"},
			},
			5: {
				{Artist: "Liturgy", Album: "As the Blood of God Bursts the Veins of Time (EP)"},
			},
			7: {
				{Artist: "Blind Illusion", Album: "Wrath of the Gods"},
				{Artist: "Borealis", Album: "Illusions"},
				{Artist: "Charlotte Wessels", Album: "Tales from Six Feet Under, Vol.II"},
				{Artist: "Counterparts", Album: "A Eulogy for Those Still Here"},
				{Artist: "The Cult", Album: "Under the Midnight Sun"},
				{Artist: "Ellefson–Soto", Album: "Vacation in the Underworld"},
				{Artist: "Goatwhore", Album: "Angels Hung from the Arches of Heaven"},
				{Artist: "King Gizzard & the Lizard Wizard", Album: "Ice, Death, Planets, Lungs, Mushrooms and Lava"},
				{Artist: "Lamb of God", Album: "Omens"},
				{Artist: "Lost Society", Album: "If the Sky Came Down"},
				{Artist: "Queensrÿche", Album: "Digital Noise Alliance"},
				{Artist: "Wednesday 13", Album: "Horrifier"},
			},
			12: {
				{Artist: "King Gizzard & the Lizard Wizard", Album: "Laminated Denim"},
			},
			14: {
				{Artist: "Alter Bridge", Album: "Pawns & Kings"},
				{Artist: "Bloody Hammers", Album: "Washed in the Blood"},
				{Artist: "Dragonland", Album: "The Power of the Nightstar"},
				{Artist: "Eleine", Album: "Acoustic in Hell (EP)"},
				{Artist: "Gun", Album: "The Calton Songs"},
				{Artist: "Lorna Shore", Album: "Pain Remains"},
				{Artist: "Nothing More", Album: "Spirits"},
				{Artist: "Outline in Color", Album: "Coast Is Clear"},
				{Artist: "Skid Row", Album: "The Gang's All Here"},
				{Artist: "Sleeping with Sirens", Album: "Complete Collapse"},
				{Artist: "Varials", Album: "Scars for You to Remember"},
				{Artist: "We Came as Romans", Album: "Darkbloom"},
			},
			21: {
				{Artist: "Architects", Album: "The Classic Symptoms of a Broken Spirit"},
				{Artist: "Avantasia", Album: "A Paranormal Evening with the Moonflower Society"},
				{Artist: "Avatarium", Album: "Death, Where Is Your Sting"},
				{Artist: "Black Veil Brides", Album: "The Mourning (EP)"},
				{Artist: "Brutus", Album: "Unison Life"},
				{Artist: "Exhumed", Album: "To the Dead"},
				{Artist: "Gothminister", Album: "Pandemonium"},
				{Artist: "In This Moment", Album: "Blood 1983 (EP)"},
				{Artist: "Sahg", Album: "Born Demon"},
				{Artist: "Serj Tankian", Album: "Perplex Cities (EP)"},
				{Artist: "Stryper", Album: "The Final Battle"},
				{Artist: "Ugly Kid Joe", Album: "Rad Wings of Destiny"},
				{Artist: "WarCry", Album: "Daimon"},
				{Artist: "White Skull", Album: "Metal Never Rusts"},
			},
			24: {
				{Artist: "Galahad", Album: "The Last Great Adventurer"},
			},
			26: {
				{Artist: "Fear, and Loathing in Las Vegas", Album: "Cocoon for the Golden Future"},
			},
			28: {
				{Artist: "Brant Bjork", Album: "Bougainvillea Suite"},
				{Artist: "Darkthrone", Album: "Astral Fortress"},
				{Artist: "Dead Cross", Album: "II"},
				{Artist: "Defleshed", Album: "Grind Over Matter"},
				{Artist: "Demon Hunter", Album: "Exile"},
				{Artist: "Despised Icon", Album: "Déterré (EP)"},
				{Artist: "Dr.Acula", Album: "Dr.Acula"},
				{Artist: "Fear Factory", Album: "Recoded (remix album)"},
				{Artist: "Fire from the Gods", Album: "Soul Revolution"},
				{Artist: "Fit for a King", Album: "The Hell We Create"},
				{Artist: "Joe Lynn Turner", Album: "Belly of the Beast"},
				{Artist: "King Gizzard & the Lizard Wizard", Album: "Changes"},
				{Artist: "Royal Hunt", Album: "Dystopia – Part II"},
				{Artist: "Sodom", Album: "40 Years at War – The Greatest Hell of Sodom (compilation album)"},
				{Artist: "Therion", Album: "Leviathan II"},
			},
		},
		November: models.Releases{
			4: {
				{Artist: "96 Bitter Beings", Album: "Synergy Restored"},
				{Artist: "Black Anvil", Album: "Regenesis"},
				{Artist: "Dayseeker", Album: "Dark Sun"},
				{Artist: "Depresszió", Album: "Vissza a Földre"},
				{Artist: "Devin Townsend", Album: "Lightwork"},
				{Artist: "Disillusion", Album: "Ayam"},
				{Artist: "Frank Bello", Album: "Then I'm Gone (EP)"},
				{Artist: "Ingested", Album: "Ashes Lie Still"},
				{Artist: "Voivod", Album: "Ultraman (EP)"},
			},
			8: {
				{Artist: "Vinnie Moore", Album: "Double Exposure"},
			},
			11: {
				{Artist: "Arallu", Album: "Death Covenant"},
				{Artist: "Chelsea Grin", Album: "Suffer in Hell"},
				{Artist: "Drudkh", Album: "Всі належать ночі"},
				{Artist: "Enuff Z'Nuff", Album: "Finer Than Sin"},
				{Artist: "Epica", Album: "The Alchemy Project (EP)"},
				{Artist: "He Is Legend", Album: "Endless Hallway"},
				{Artist: "Kampfar", Album: "Til Klovers Takt"},
				{Artist: "Last in Line", Album: "A Day in the Life (EP)"},
				{Artist: "Leatherwolf", Album: "Kill the Hunted"},
				{Artist: "Ring of Fire", Album: "Gravity"},
				{Artist: "Xentrix", Album: "Seven Words"},
			},
			18: {
				{Artist: "16", Album: "Into Dust"},
				{Artist: "Aurora Borealis", Album: "Prophecy Is the Mold in Which History Is Poured"},
				{Artist: "Autograph", Album: "Beyond"},
				{Artist: "Candlemass", Album: "Sweet Evil Sun"},
				{Artist: "Disturbed", Album: "Divisive"},
				{Artist: "Nickelback", Album: "Get Rollin'"},
				{Artist: "Ronnie Atkins", Album: "Symphomaniac (EP)"},
				{Artist: "Saint Asonia", Album: "Extrovert (EP)"},
				{Artist: "Soen", Album: "Atlantis (live album)"},
				{Artist: "Tallah", Album: "The Generation of Danger"},
				{Artist: "Threshold", Album: "Dividing Lines"},
				{Artist: "U.D.O.", Album: "The Legacy (compilation album)"},
				{Artist: "Wolves at the Gate", Album: "Lowborn (EP)"},
			},
			25: {
				{Artist: "Elder", Album: "Innate Passage"},
				{Artist: "Hibernus Mortis", Album: "The Monoliths of Cursed Slumber"},
				{Artist: "In the Woods...", Album: "Diversum"},
				{Artist: "Judicator", Album: "The Majesty of Decay"},
				{Artist: "The Last Ten Seconds of Life", Album: "Disquisition on an Execution (EP)"},
				{Artist: "Leather", Album: "We Are the Chosen"},
				{Artist: "Lee Aaron", Album: "Elevate"},
				{Artist: "Ofermod", Album: "Ofermodian Litanies (mini-album)"},
				{Artist: "Sword", Album: "III"},
			},
			28: {
				{Artist: "Necrodeath", Album: "Singin' in the Pain"},
			},
		},
		December: models.Releases{
			2: {
				{Artist: "Amberian Dawn", Album: "Take a Chance – A Metal Tribute to ABBA (covers album)"},
				{Artist: "Deströyer 666", Album: "Never Surrender"},
				{Artist: "Eisregen", Album: "Wiedergänger (EP)"},
				{Artist: "Hammers of Misfortune", Album: "Overtaker"},
			},
			9: {
				{Artist: "Lionheart", Album: "Welcome to the West Coast III"},
				{Artist: "Ripper", Album: "Return to Death Row (EP)"},
				{Artist: "Serenity", Album: "Memoria (live album)"},
			},
			14: {
				{Artist: "Nemophila", Album: "Seize the Fate"},
			},
			15: {
				{Artist: "Rotting Christ", Album: "The Apocryphal Spells, Vol.I (EP)"},
				{Artist: "Rotting Christ", Album: "The Apocryphal Spells, Vol.II (EP)"},
			},
			22: {
				{Artist: "Rudra", Album: "Eight Mahavidyas"},
			},
			25: {
				{Artist: "Snowy Shaw", Album: "This Is Heavy Metal, Plain & Simple (compilation album)"},
			},
			30: {
				{Artist: "Lord of the Lost", Album: "Blood & Glitter"},
				{Artist: "Satanic Warmaster", Album: "Aamongandr"},
			},
		},
	}

	_, fileName, _, _ := runtime.Caller(0)
	f, err := os.Open(path.Dir(fileName) + "/testdata/" + "wiki2022.html")
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()
	doc, _ := goquery.NewDocumentFromReader(f)

	got := scraper.ScrapeMetalReleases(doc)

	if diff := diffCalendar(got, *want); diff != "" {
		t.Log("\n" + diff)
		t.Fail()
	}
}

func diffCalendar(got, want models.Calendar) string {
	return diffMonth(time.January, got.January, want.January) +
		diffMonth(time.February, got.February, want.February) +
		diffMonth(time.March, got.March, want.March) +
		diffMonth(time.April, got.April, want.April) +
		diffMonth(time.May, got.May, want.May) +
		diffMonth(time.June, got.June, want.June) +
		diffMonth(time.July, got.July, want.July) +
		diffMonth(time.August, got.August, want.August) +
		diffMonth(time.September, got.September, want.September) +
		diffMonth(time.October, got.October, want.October) +
		diffMonth(time.November, got.November, want.November) +
		diffMonth(time.December, got.December, want.December)
}

func diffMonth(month time.Month, got, want models.Releases) string {
	var diff string

	gotKeys := maps.Keys(got)
	wantKeys := maps.Keys(want)
	if len(gotKeys) != len(wantKeys) {
		return fmt.Sprintf("%s: missing days - got %v but want %v\n", month, gotKeys, wantKeys)
	}

	for _, day := range gotKeys {
		if len(got[day]) != len(want[day]) {
			diff += fmt.Sprintf("%s %d: missing releases\n\tgot %+v\n\tbut want\n\t%+v\n", month, day, got[day], want[day])
		}

		slices.SortFunc(got[day], func(a, b models.Release) bool {
			return a.Artist < b.Artist && a.Album < b.Album
		})

		slices.SortFunc(want[day], func(a, b models.Release) bool {
			return a.Artist < b.Artist && a.Album < b.Album
		})

		for i, release := range got[day] {
			if release.Artist == "" || release.Album == "" {
				diff += fmt.Sprintf("%s %d: missing release info - got %+v but want %+v\n", month, day, got[day][i], want[day][i])
			}
		}
	}

	return diff
}
