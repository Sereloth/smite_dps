#![allow(dead_code)]
#![allow(unused_variables)]

use fltk::{app, button::Button, prelude::*, window::Window,*};
use itertools::Itertools;



struct Item
{
    name: &'static str,
    magical_power: f32,
    physical_power: f32,
    flat_pen: f32,
    percent_pen: f32,
    attack_speed: f32,
    crit_chance: f32,
    lifesteal: f32,
    cdr: f32,
    mana:f32,
    health: f32,
    phys_prot: f32,
    magical_prot: f32,
    mp5: f32,
    hp5: f32,
    ccr: f32,
    move_speed: f32,
    gold: f32,
}

struct Build
{
    names: [&'static str;6],
    magical_power: f32,
    physical_power: f32,
    flat_pen: f32,
    percent_pen: f32,
    attack_speed: f32,
    crit_chance: f32,
    lifesteal: f32,
    cdr: f32,
    mana:f32,
    health: f32,
    phys_prot: f32,
    magical_prot: f32,
    mp5: f32,
    hp5: f32,
    ccr: f32,
    move_speed: f32,
    gold: f32,
}

//base stats are stats at level 1
struct God
{
    name: &'static str,
    base_health: f32,
    health_per_level : f32,
    base_mana: f32,
    mana_per_level : f32,
    base_as: f32,
    as_per_level : f32,
    base_auto_damage: f32,
    auto_damage_per_level : f32,
    auto_progression: [f32;7],
    base_phys_prots: f32,
    phys_prots_per_level : f32,
    base_magical_prots: f32,
    magical_prots_per_level : f32,
    base_hp5: f32,
    hp5_per_level : f32,
    base_mp5: f32,
    mp5_per_level : f32,
    class : &'static str,
}


//-------------------------------------------------------------------------
//Items

static EMPTY:Item = Item
{
    name: "Empty",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 0.0,
};

static GRIFFONWING:Item = Item
{
    name: "Griffonwing Earrings",
    magical_power: 80.0,
    physical_power: 55.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static SPHINX:Item = Item
{
    name: "Sphinx's Baubles",
    magical_power: 80.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2550.0,
};

//-------------------------------------------------------------------------
//Starters

static MAGICAL_STARTERS:[&Item;33] = [&EMPTY,&SANDS,&PENDULUM,&ALTERNATE_TIMELINE,&CONDUIT,&ARCHMAGES,&FOCUS,
                                        &VAMP_SHROUD,&BLOODSOAKED,&SACRIFICIAL,&DEATHS_TOLL,&DEATHS_EMBRACE,&DEATHS_TEMPER,
                                        &GILDED,&DIAMOND,&ORNATE,&BUMBAS_DAGGER,&BUMBAS_SPEAR,&BUMBAS_HAMMER,&EYE,&SEER,&PROTECTOR,
                                        &MANIKIN_SCEPTER,&MANIKINS_MACE,&MHB,&TAINTED_STEEL,&TAINTED_AMULET,&TAINTED_BREASTPLATE,
                                        &FIGHTERS,&RANGDAS,&ANIMOSITY,&SPARTAN,&GLEAMING_CUFFS];

static PHYSICAL_STARTERS:[&Item;29] = [&EMPTY,&DEATHS_TOLL,&DEATHS_EMBRACE,&DEATHS_TEMPER,&GILDED,&DIAMOND,&ORNATE,&LEATHER_COWL,
                                        &HUNTERS_COWL,&BLUESTONE_PENDANT,&BLUESTONE_BROOCH,&REDSTONE,&BUMBAS_DAGGER,&BUMBAS_SPEAR,
                                        &BUMBAS_HAMMER,&EYE,&SEER,&PROTECTOR,&MANIKIN_SCEPTER,&MANIKINS_MACE,&MHB,&TAINTED_STEEL,
                                        &TAINTED_AMULET,&TAINTED_BREASTPLATE,&FIGHTERS,&RANGDAS,&ANIMOSITY,&SPARTAN,&GLEAMING_CUFFS];
   
                                        
static SANDS:Item = Item
{
    name: "Sands of Time",
    magical_power: 25.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 700.0,
};


static PENDULUM:Item = Item
{
    name: "Pendulum of Ages",
    magical_power: 190.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.2,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static ALTERNATE_TIMELINE:Item = Item
{
    name: "Alternate Timeline",
    magical_power: 70.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 45.0,
    magical_prot: 45.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static CONDUIT:Item = Item
{
    name: "Conduit Gem",
    magical_power: 25.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 100.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 700.0,
};

static ARCHMAGES:Item = Item
{
    name: "Archmage's Gem",
    magical_power: 120.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 25.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static FOCUS:Item = Item
{
    name: "Gem of Focus",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static VAMP_SHROUD:Item = Item
{
    name: "Vampiric Shroud",
    magical_power: 35.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.04,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static BLOODSOAKED:Item = Item
{
    name: "Blood-Soaked Shroud",
    magical_power: 110.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana: 0.0,
    health: 350.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.15,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static SACRIFICIAL:Item = Item
{
    name: "Sacrificial Shroud",
    magical_power: 125.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 50.0,
    magical_prot: 0.0,
    mp5: 0.15,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static DEATHS_TOLL:Item = Item
{
    name: "Death's Toll",
    magical_power: 35.0,
    physical_power: 18.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 75.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static DEATHS_EMBRACE:Item = Item
{
    name: "Death's Embrace",
    magical_power: 110.0,
    physical_power: 65.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static DEATHS_TEMPER:Item = Item
{
    name: "Death's Temper",
    magical_power: 75.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 100.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static GILDED:Item = Item
{
    name: "Gilded Arrow",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 50.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 7.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static DIAMOND:Item = Item
{
    name: "Diamond Arrow",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.1,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static ORNATE:Item = Item
{
    name: "Ornate Arrow",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.35,
    crit_chance: 0.35,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static LEATHER_COWL:Item = Item
{
    name: "Leather Cowl",
    magical_power: 0.0,
    physical_power: 18.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.1,
    crit_chance: 0.0,
    lifesteal: 0.04,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 5.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static HUNTERS_COWL:Item = Item
{
    name: "Hunter's Cowl",
    magical_power: 0.0,
    physical_power: 70.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.1,
    crit_chance: 0.0,
    lifesteal: 0.2,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static BLUESTONE_PENDANT:Item = Item
{
    name: "Bluestone Pendant",
    magical_power: 0.0,
    physical_power: 15.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 15.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 700.0,
};

static BLUESTONE_BROOCH:Item = Item
{
    name: "Bluestone Brooch",
    magical_power: 0.0,
    physical_power: 45.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 30.0,
    hp5: 30.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static REDSTONE:Item = Item
{
    name: "Corrupted Bluestone",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static BUMBAS_DAGGER:Item = Item
{
    name: "Bumba's Dagger",
    magical_power: 15.0,
    physical_power: 10.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 50.0,
    health: 50.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 600.0,
};

static BUMBAS_SPEAR:Item = Item
{
    name: "Bumba's Spear",
    magical_power: 105.0,
    physical_power: 70.0,
    flat_pen: 0.0,
    percent_pen: 0.1,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static BUMBAS_HAMMER:Item = Item
{
    name: "Bumba's Hammer",
    magical_power: 80.0,
    physical_power: 60.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static EYE:Item = Item
{
    name: "Eye of the Jungle",
    magical_power: 30.0,
    physical_power: 15.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.07,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 10.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static SEER:Item = Item
{
    name: "Seer of the Jungle",
    magical_power: 100.0,
    physical_power: 70.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 40.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static PROTECTOR:Item = Item
{
    name: "Protector of the Jungle",
    magical_power: 100.0,
    physical_power: 70.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.35,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 35.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static MANIKIN_SCEPTER:Item = Item
{
    name: "Manikin Scepter",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 10.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 750.0,
};

static MANIKINS_MACE:Item = Item
{
    name: "Manikin Mace",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 100.0,
    phys_prot: 30.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static MHB:Item = Item
{
    name: "Manikin Hidden Blade",
    magical_power: 90.0,
    physical_power: 60.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 30.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static TAINTED_STEEL:Item = Item
{
    name: "Tainted Steel",
    magical_power: 25.0,
    physical_power: 15.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 15.0,
    magical_prot: 15.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 600.0,
};

static TAINTED_AMULET:Item = Item
{
    name: "Tainted Amulet",
    magical_power: 50.0,
    physical_power: 30.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 30.0,
    magical_prot: 90.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static TAINTED_BREASTPLATE:Item = Item
{
    name: "Tainted Breastplate",
    magical_power: 75.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 80.0,
    magical_prot: 30.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static FIGHTERS:Item = Item
{
    name: "Fighter's Mask",
    magical_power: 15.0,
    physical_power: 10.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 10.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 650.0,
};

static RANGDAS:Item = Item
{
    name: "Rangda's Mask",
    magical_power: 80.0,
    physical_power: 60.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static ANIMOSITY:Item = Item
{
    name: "Axe of Animosity",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 35.0,
    magical_prot: 35.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.3,
    move_speed: 0.0,
    gold: 2150.0,
};

static SPARTAN:Item = Item
{
    name: "Spartan Flag",
    magical_power: 65.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 1850.0,
};

static GLEAMING_CUFFS:Item = Item
{
    name: "Gleaming Ear Cuffs",
    magical_power: 30.0,
    physical_power: 20.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 1300.0,
};

//-------------------------------------------------------------------------
//Acorns

static EVERGREEN:Item = Item
{
    name: "Evergreen Acorn",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.6,
    gold: 2050.0,
};

static THICKBARK:Item = Item
{
    name: "Thickbark Acorn",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.6,
    gold: 2050.0,
};

static BRISTLEBUSH:Item = Item
{
    name: "Bristlebush Acorn",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.25,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.6,
    gold: 2050.0,
};

static THISTLETHORN:Item = Item
{
    name: "Thistlethorn Acorn",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.6,
    gold: 2050.0,
};

//-------------------------------------------------------------------------
//Magical Items

static MAGICAL_ITEMS:[&Item;37] = [&EMPTY,&DIVINE,&MAGUS,&DESO,&OBBY,&TAHUTI,&CALAM_TAHUTI,&CHRONOS_PENDANT,&CHARONS,
                                    &DOOM_ORB,&MYRDDIN,&BOOK_OF_THOTH,&TABLET,&POLY,&REAVER,
                                    &PYTHAGS,&SOUL_GEM,&BANCROFTS,&NIMBLE_BANCROFTS,&BANCROFTS_CLAW,&TYPHONS,&BINDING,
                                    &ASCLEPIUS,&LAST_GASP,&REJUV_HEART,&ISOLATION,&ESTAFF,
                                    &WARLOCKS,&DEMONIC_GRIP,&TELKHINES,&CYCLOP_RING,&HASTENED_RING,&GRIFFONWING,&SPHINX,&TOXICBLADE,&BERSERKERS,&GLADS];

static DIVINE:Item = Item
{
    name: "Divine Ruin",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 15.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2550.0,
};

static MAGUS:Item = Item
{
    name: "Spear of the Magus",
    magical_power: 110.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.08,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2450.0,
};

static DESO:Item = Item
{
    name: "Spear of Desolation",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 15.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static OBBY:Item = Item
{
    name: "Obsidian Shard",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.16,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static TAHUTI:Item = Item
{
    name: "Rod of Tahuti",
    magical_power: 140.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 30.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2850.0,
};

static CALAM_TAHUTI:Item = Item
{
    name: "Calamitous Rod of Tahuti",
    magical_power: 140.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 30.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 3450.0,
};

static CHRONOS_PENDANT:Item = Item
{
    name: "Chronos' Pendant",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.2,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static CHARONS:Item = Item
{
    name: "Charon's Coin",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.16,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 35.0,
    hp5: 35.0,
    ccr: 0.0,
    move_speed: 0.08,
    gold: 2400.0,
};

static DOOM_ORB:Item = Item
{
    name: "Doom Orb",
    magical_power: 130.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 25.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.06,
    gold: 2850.0,
};

static MYRDDIN:Item = Item
{
    name: "Staff of Myrddin",
    magical_power: 105.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static BOOK_OF_THOTH:Item = Item
{
    name: "Book of Thoth",
    magical_power: 70.0,
    physical_power: 0.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 775.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static TABLET:Item = Item
{
    name: "Tablet of Destinies",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:300.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2650.0,
};

static POLY:Item = Item
{
    name: "Polynomicon",
    magical_power: 95.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.08,
    cdr: 0.0,
    mana:300.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static REAVER:Item = Item
{
    name: "Soul Reaver",
    magical_power: 95.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:300.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2650.0,
};

static PYTHAGS:Item = Item
{
    name: "Pythagorem's Piece",
    magical_power: 80.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.16,
    cdr: 0.1,
    mana:0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static SOUL_GEM:Item = Item
{
    name: "Soul Gem",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.08,
    cdr: 0.1,
    mana:0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static BANCROFTS:Item = Item
{
    name: "Bancroft's Talon",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana:200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static NIMBLE_BANCROFTS:Item = Item
{
    name: "Nimble Bancroft's Talon",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana:200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 3100.0,
};

static BANCROFTS_CLAW:Item = Item
{
    name: "Bancroft's Claw",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana:200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 3100.0,
};

static TYPHONS:Item = Item
{
    name: "Typhon's Fang",
    magical_power: 85.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.16,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana:200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2650.0,
};

static BINDING:Item = Item
{
    name: "Stone of Binding",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 40.0,
    magical_prot: 40.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static ISOLATION:Item = Item
{
    name: "Gem of Isolation",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.0,
    gold: 2700.0,
};

static ESTAFF:Item = Item
{
    name: "Ethereal Staff",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2600.0,
};


static WARLOCKS:Item = Item
{
    name: "Warlock's Staff",
    magical_power: 155.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:200.0,
    health: 225.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static DEMONIC_GRIP:Item = Item
{
    name: "Demonic Grip",
    magical_power: 75.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static TELKHINES:Item = Item
{
    name: "Telkhines Ring",
    magical_power: 60.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.06,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static HASTENED_RING:Item = Item
{
    name: "Hastened Ring",
    magical_power: 70.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2700.0,
};

static CYCLOP_RING :Item = Item
{
    name: "Cyclopean Ring",
    magical_power: 70.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static ASCLEPIUS:Item = Item
{
    name: "Rod of Asclepius",
    magical_power: 90.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 250.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static REJUV_HEART:Item = Item
{
    name: "Rejuvenating Heart",
    magical_power: 70.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 300.0,
    health: 300.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2450.0,
};

static LAST_GASP:Item = Item
{
    name: "Last Gasp",
    magical_power: 100.0,
    physical_power: 0.0,
    flat_pen: 15.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 200.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};


//-------------------------------------------------------------------------
//Physical Items
static PHYSICAL_ITEMS:[&Item;46] = [&EMPTY,&BRAWLERS,&JOTUNNS,&CRUSHER,&TITANS,&DOMINANCE,&TRANS,&HEARTSEEKER,&HYDRAS,&ATALANTAS,&SILVERBRANCH,
                                    &OBOW,&DEVOS,&BLOODFORGE,&SOUL_NOMMER,&EXE,&HEAVY_EXE,&FEROCIOUS_EXE,&ASI,&QINS,&DEATHBRINGER,&RAGE,&BOOMERANG,&WIND_DEMON,
                                    &FAILNOT,&ARONDIGHT,&GRIFFONWING,&SPHINX,&SHADOWDRINKER,&SERRATED,&GOLDEN_BLADE,&HASTENED_KATANA,
                                    &FROSTBOUND,&RUNEFORGED,&DAWNBRINGER,&BERSERKERS,&GLADS,&BINDING,
                                    &CAD_SHIELD,&SEKHMETS,&VITAL_AMP,&TOXICBLADE,&EVERGREEN,&THISTLETHORN,&BRISTLEBUSH,&THICKBARK];

static BRAWLERS:Item = Item
{
    name: "Brawler's Beat Stick",
    magical_power: 0.0,
    physical_power: 55.0,
    flat_pen: 15.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static JOTUNNS:Item = Item
{
    name: "Jotunn's Wrath",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.2,
    mana: 150.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static CRUSHER:Item = Item
{
    name: "The Crusher",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static TITANS:Item = Item
{
    name: "Titan's Bane",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 0.0,
    percent_pen: 0.16,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static DOMINANCE:Item = Item
{
    name: "Dominance",
    magical_power: 0.0,
    physical_power: 55.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static TRANS:Item = Item
{
    name: "Transcendence",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 1050.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static HEARTSEEKER:Item = Item
{
    name: "Heartseeker",
    magical_power: 0.0,
    physical_power: 65.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2750.0,
};

static HYDRAS:Item = Item
{
    name: "Hydra's Lament",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 200.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static ATALANTAS:Item = Item
{
    name: "Atalanta's Bow",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static SILVERBRANCH:Item = Item
{
    name: "Silverbranch Bow",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2550.0,
};

static OBOW:Item = Item
{
    name: "Odysseus' Bow",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static DEVOS:Item = Item
{
    name: "Devourer's Gauntlet",
    magical_power: 0.0,
    physical_power: 70.0,
    flat_pen: 15.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.15,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static BLOODFORGE:Item = Item
{
    name: "Bloodforge",
    magical_power: 0.0,
    physical_power: 75.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2600.0,
};

static SOUL_NOMMER:Item = Item
{
    name: "Soul Eater",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static EXE:Item = Item
{
    name: "The Executioner",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static HEAVY_EXE:Item = Item
{
    name: "The Heavy Executioner",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2800.0,
};

static FEROCIOUS_EXE:Item = Item
{
    name: "The Ferocious Executioner",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2800.0,
};


static ASI:Item = Item
{
    name: "Asi",
    magical_power: 0.0,
    physical_power: 55.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.0,
    lifesteal: 0.12,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2450.0,
};

static QINS:Item = Item
{
    name: "Qin's Sais",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static DEATHBRINGER:Item = Item
{
    name: "Deathbringer",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.3,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2900.0,
};

static RAGE:Item = Item
{
    name: "Rage",
    magical_power: 0.0,
    physical_power: 55.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.45,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static BOOMERANG:Item = Item
{
    name: "Bladed Boomerang",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.2,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static WIND_DEMON:Item = Item
{
    name: "Demon Blade",
    magical_power: 0.0,
    physical_power: 25.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.2,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};

static FAILNOT:Item = Item
{
    name: "Fail-not",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.08,
    attack_speed: 0.0,
    crit_chance: 0.2,
    lifesteal: 0.0,
    cdr: 0.2,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2650.0,
};

static ARONDIGHT:Item = Item
{
    name: "Arondight",
    magical_power: 0.0,
    physical_power: 75.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2600.0,
};

static SHADOWDRINKER:Item = Item
{
    name: "Shadowdrinker",
    magical_power: 0.0,
    physical_power: 50.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2300.0,
};

static SERRATED:Item = Item
{
    name: "Serrated Edge",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.16,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.1,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2500.0,
};

static GOLDEN_BLADE:Item = Item
{
    name: "Golden Blade",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.2,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2150.0,
};

static HASTENED_KATANA:Item = Item
{
    name: "Hastened Katana",
    magical_power: 0.0,
    physical_power: 25.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 0.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2450.0,
};

static FROSTBOUND:Item = Item
{
    name: "Frostbound Hammer",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 350.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static RUNEFORGED:Item = Item
{
    name: "Runeforged Hammer",
    magical_power: 0.0,
    physical_power: 35.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 15.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static DAWNBRINGER:Item = Item
{
    name: "Dawnbringer",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2400.0,
};



static BERSERKERS:Item = Item
{
    name: "Berserker's Shield",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.25,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 150.0,
    phys_prot: 60.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static GLADS:Item = Item
{
    name: "Gladiator's Shield",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 200.0,
    phys_prot: 50.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 25.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static CAD_SHIELD:Item = Item
{
    name: "Caduceus Club",
    magical_power: 0.0,
    physical_power: 30.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.04,
    gold: 2400.0,
};

static VITAL_AMP:Item = Item
{
    name: "Vital Amplifier",
    magical_power: 0.0,
    physical_power: 40.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2450.0,
};

static SEKHMETS:Item = Item
{
    name: "Sekhmet's Scepter",
    magical_power: 0.0,
    physical_power: 45.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2500.0,
};




//-------------------------------------------------------------------------
//Tank Items

static TANK_ITEMS:[&Item;41] = [&WINGED_BLADE,&RELIC_DAGGER,&LOTUS,&TOXICBLADE,&RENEWAL,&THEBES,&GAIA,&SPIRIT_ROBE,&MANTLE,&MAGIS
                                    ,&PRIDWEN,&GENJIS,&ONI_HUNTERS,&SHOGUNS,&PESTI,&HEARTWARD,&TALISMAN,&BREASTPLATE,&REGROWTH,&SPECTRAL,
                                     &CONTAGION,&SOV,&MYSTICAL,&MIDGARDIAN,&EMPERORS,&LOTUS,&BINDING,&BERSERKERS,&GLADS,&PHALANX
                                    ,&CAD_SHIELD,&FROSTBOUND,&RUNEFORGED,&MANTICORE,&FAE_BLESSED,&ABSOLUTION,&ABYSSAL,&ARCHDRUID,&EROSION,&PROPHETIC,
                                     &CANNONEER];




static WINGED_BLADE:Item = Item
{
    name: "Winged Blade",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 25.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.07,
    gold: 1850.0,
};

static RELIC_DAGGER:Item = Item
{
    name: "Relic Dagger",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 350.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.07,
    gold: 1850.0,
};

static LOTUS:Item = Item
{
    name: "Lotus Sickle",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 300.0,
    phys_prot: 25.0,
    magical_prot: 0.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2000.0,
};

static TOXICBLADE:Item = Item
{
    name: "Toxic Blade",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 10.0,
    percent_pen: 0.0,
    attack_speed: 0.15,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 100.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.07,
    gold: 2400.0,
};


static RENEWAL:Item = Item
{
    name: "Mail of Renewal",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 350.0,
    phys_prot: 20.0,
    magical_prot: 20.0,
    mp5: 0.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static THEBES:Item = Item
{
    name: "Gauntlet of Thebes",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 65.0,
    magical_prot: 65.0,
    mp5: 0.0,
    hp5: 15.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static GAIA:Item = Item
{
    name: "Stone of Gaia",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 400.0,
    phys_prot: 0.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 25.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static SPIRIT_ROBE:Item = Item
{
    name: "Spirit Robe",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 40.0,
    magical_prot: 40.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2400.0,
};

static MANTLE:Item = Item
{
    name: "Mantle of Discord",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 0.0,
    phys_prot: 55.0,
    magical_prot: 55.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2900.0,
};

static MAGIS:Item = Item
{
    name: "Magi's Cloak",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 10.0,
    mana:0.0,
    health: 0.0,
    phys_prot: 30.0,
    magical_prot: 30.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static PRIDWEN:Item = Item
{
    name: "Pridwen",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.2,
    mana:0.0,
    health: 0.0,
    phys_prot: 30.0,
    magical_prot: 30.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static GENJIS:Item = Item
{
    name: "Genji's Guard",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 70.0,
    mp5: 40.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static ONI_HUNTERS:Item = Item
{
    name: "Oni Hunter's Garb",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 200.0,
    phys_prot: 0.0,
    magical_prot: 60.0,
    mp5: 30.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2100.0,
};

static SHOGUNS:Item = Item
{
    name: "Shogun's Kusari",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:0.0,
    health: 150.0,
    phys_prot: 0.0,
    magical_prot: 50.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2200.0,
};

static PESTI:Item = Item
{
    name: "Pestilence",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 80.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static HEARTWARD:Item = Item
{
    name: "Heartward Amulent",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 75.0,
    mp5: 30.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static TALISMAN:Item = Item
{
    name: "Talisman of Energy",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 300.0,
    phys_prot: 0.0,
    magical_prot: 60.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2150.0,
};

static BREASTPLATE:Item = Item
{
    name: "Breastplate of Valor",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.2,
    mana: 300.0,
    health: 0.0,
    phys_prot: 65.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static REGROWTH:Item = Item
{
    name: "Breastplate of Regrowth",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana:300.0,
    health: 300.0,
    phys_prot: 55.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold:2250.0,
};


static SPECTRAL:Item = Item
{
    name: "Spectral Armor",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 300.0,
    health: 200.0,
    phys_prot: 60.0,
    magical_prot: 0.0,
    mp5: 10.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static CONTAGION:Item = Item
{
    name: "Contagion",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 250.0,
    health: 150.0,
    phys_prot: 60.0,
    magical_prot: 0.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static SOV:Item = Item
{
    name: "Sovereignty",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 55.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 25.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2100.0,
};

static MYSTICAL:Item = Item
{
    name: "Mystical Mail",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 250.0,
    phys_prot: 40.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2450.0,
};

static MIDGARDIAN:Item = Item
{
    name: "Midgardian Mail",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 300.0,
    phys_prot: 50.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static EMPERORS:Item = Item
{
    name: "Emperor's Armor",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana:0.0,
    health: 300.0,
    phys_prot: 60.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2250.0,
};

static MANTICORE:Item = Item
{
    name: "Manticore's Spikes",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 35.0,
    magical_prot: 35.0,
    mp5: 20.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2400.0,
};

static FAE_BLESSED:Item = Item
{
    name: "Fae-Blessed Hoops",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 300.0,
    phys_prot: 0.0,
    magical_prot: 50.0,
    mp5: 40.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2300.0,
};

static ABSOLUTION:Item = Item
{
    name: "Absolution",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 250.0,
    phys_prot: 0.0,
    magical_prot: 70.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.2,
    move_speed: 0.0,
    gold: 2250.0,
};

static ABYSSAL:Item = Item
{
    name: "Abyssal Stone",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 40.0,
    magical_prot: 40.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.0,
    gold: 2350.0,
};

static ARCHDRUID:Item = Item
{
    name: "Archdruid's Fury",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 35.0,
    magical_prot: 35.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.1,
    move_speed: 0.0,
    gold: 2400.0,
};

static EROSION:Item = Item
{
    name: "Erosion",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 350.0,
    phys_prot: 30.0,
    magical_prot: 30.0,
    mp5: 15.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static PHALANX:Item = Item
{
    name: "Phalanx",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.3,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 200.0,
    phys_prot: 60.0,
    magical_prot: 0.0,
    mp5: 0.0,
    hp5: 20.0,
    ccr: 0.1,
    move_speed: 0.0,
    gold: 2200.0,
};

static PROPHETIC:Item = Item
{
    name: "Prophetic Cloak",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.1,
    mana: 0.0,
    health: 150.0,
    phys_prot: 70.0,
    magical_prot: 70.0,
    mp5: 0.0,
    hp5: 0.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2350.0,
};

static CANNONEER:Item = Item
{
    name: "Cannoneer's Cuirass",
    magical_power: 0.0,
    physical_power: 0.0,
    flat_pen: 0.0,
    percent_pen: 0.0,
    attack_speed: 0.0,
    crit_chance: 0.0,
    lifesteal: 0.0,
    cdr: 0.0,
    mana: 0.0,
    health: 300.0,
    phys_prot: 30.0,
    magical_prot: 30.0,
    mp5: 0.0,
    hp5: 20.0,
    ccr: 0.0,
    move_speed: 0.0,
    gold: 2000.0,
};



static GODS:[&God;126] = [&AMC,&ANHUR,&APOLLO,&ARTEMIS,&CERN,&CHARYBDIS,&CHERNO,&CHIRON,&CUPID,&DANZA,&HACHI,&HEIM,
                        &HOUYI,&ISHTAR,&IZANAMI,&JING,&MANTI,&MEDUSA,&NEITH,&RAMA,&SKADI,&ULLR,&XBAL,&AGNI,&AHPUCH,&ANUBIS,&AOKUANG,
                        &APHRO,&BABA,&BARON,&CHANGE,&CHRONOS,&DISCO,&ESET,&FREYA,&HADES,&HEBO,&HEL,&HERA,&IX,&JANUS,
                        &KUKU,&MERLIN,&MORGANA,&NOX,&NUWA,&OLORUN,&PERSE,&POSEIDON,&RA,&RAIJIN,&SCYLLA,&SOL,&MORRI,
                        &THOTH,&TIAMAT,&VULCAN,&YUHUANG,&ZEUS,&ZHONG,&ARACHNE,&AWILIX,&BAKA,&BASTET,&CAMA,&CLIO,&DAJI,
                        &FENRIR,&HUNBATZ,&KALI,&LANCELOT,&LOKI,&MERC,&NEZHA,&NEMESIS,&PELE,&RAT,&RAVANA,&SERQET,&SET,
                        &SUSANO,&THANA,&THOR,&TSUKU,&ARES,&ARTIO,&ATHENA,&ATLAS,&BACCHUS,&CABRAKAN,&CERBERUS
                        ,&CTHULHU,&FAFNIR,&GANESHA,&GEB,&JORM,&KHEPRI,&KUMBHAKARNA,&KUZENBO
                        ,&MAUI,&SOBEK,&SYLVANUS,&TERRA,&XING,&YEMOJA,&YMIR,&ACHILLES,&AMATERASU,&BELLONA,&CHAAC
                        ,&CU_CHU,&ERLANG,&GILG,&GUAN,&HERCULES,&HORUS,&ARTHUR,&MULAN,&NIKE,&ODIN,&OSIRIS,&SHIVA,&SURTR
                        ,&WUKONG,&TYR,&VAMANA];


static ERROR:God = God //used when god name is invalid
{
    name: "Error",
    base_health: 579.0,
    health_per_level: 75.0,
    base_mana: 300.0,
    mana_per_level: 45.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 40.0,
    magical_prots_per_level: 1.2,
    base_hp5: 7.47,
    hp5_per_level: 0.47,
    base_mp5: 5.07,
    mp5_per_level: 0.37,
    class: "Error"
};


//-------------------------------------------------------------------------
//Gods MAGES

static AGNI:God = God
{
    name: "Agni",
    base_health: 579.0,
    health_per_level: 75.0,
    base_mana: 300.0,
    mana_per_level: 45.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.47,
    hp5_per_level: 0.47,
    base_mp5: 5.07,
    mp5_per_level: 0.37,
    class: "Mage"
};

static AHPUCH:God = God
{
    name: "Ah Puch",
    base_health: 709.0,
    health_per_level: 79.0,
    base_mana: 320.0,
    mana_per_level: 55.0,
    base_as: 0.87,
    as_per_level: 0.0095,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.32,
    mp5_per_level: 0.42,
    class: "Mage"
};

static ANUBIS:God = God
{
    name: "Anubis",
    base_health: 606.0,
    health_per_level: 74.0,
    base_mana: 338.0,
    mana_per_level: 58.0,
    base_as: 0.87,
    as_per_level: 0.0099,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 17.0,
    phys_prots_per_level: 2.7,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.45,
    hp5_per_level: 0.45,
    base_mp5: 5.16,
    mp5_per_level: 0.36,
    class: "Mage"
};

static AOKUANG:God = God
{
    name: "Ao Kuang",
    base_health: 646.0,
    health_per_level: 86.0,
    base_mana: 278.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.0195,
    base_auto_damage: 37.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,0.5,0.5,1.0,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.7,
    hp5_per_level: 0.7,
    base_mp5: 5.21,
    mp5_per_level: 0.41,
    class: "Mage"
};

static APHRO:God = God
{
    name: "Aphro",
    base_health: 603.0,
    health_per_level: 71.0,
    base_mana: 283.0,
    mana_per_level: 43.0,
    base_as: 0.88,
    as_per_level: 0.0095,
    base_auto_damage: 33.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.21,
    mp5_per_level: 0.41,
    class: "Mage"
};

static BABA:God = God
{
    name: "Baba Yaga",
    base_health: 637.0,
    health_per_level: 77.0,
    base_mana: 330.0,
    mana_per_level: 50.0,
    base_as: 0.96,
    as_per_level: 0.01,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.5,
    hp5_per_level: 0.5,
    base_mp5: 5.6,
    mp5_per_level: 0.6,
    class: "Mage"
};

static BARON:God = God
{
    name: "Baron Samedi",
    base_health: 616.0,
    health_per_level: 84.0,
    base_mana: 344.0,
    mana_per_level: 44.0,
    base_as: 0.91,
    as_per_level: 0.008,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.6,
    hp5_per_level: 0.6,
    base_mp5: 5.45,
    mp5_per_level: 0.45,
    class: "Mage"
};

static CHANGE:God = God
{
    name: "Chang'e",
    base_health: 738.0,
    health_per_level: 88.0,
    base_mana: 358.0,
    mana_per_level: 48.0,
    base_as: 1.01,
    as_per_level: 0.0095,
    base_auto_damage: 33.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.94,
    mp5_per_level: 0.44,
    class: "Mage"
};

static CHRONOS:God = God
{
    name: "Chronos",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 282.0,
    mana_per_level: 42.0,
    base_as: 1.01,
    as_per_level: 0.015,
    base_auto_damage: 42.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.48,
    hp5_per_level: 0.48,
    base_mp5: 4.84,
    mp5_per_level: 0.44,
    class: "Mage"
};

static DISCO:God = God
{
    name: "Discordia",
    base_health: 589.0,
    health_per_level: 71.0,
    base_mana: 331.0,
    mana_per_level: 51.0,
    base_as: 0.91,
    as_per_level: 0.01,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.45,
    hp5_per_level: 0.45,
    base_mp5: 5.43,
    mp5_per_level: 0.43,
    class: "Mage"
};

static ESET:God = God
{
    name: "Eset",
    base_health: 582.0,
    health_per_level: 71.0,
    base_mana: 331.0,
    mana_per_level: 51.0,
    base_as: 1.0,
    as_per_level: 0.0,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.23,
    mp5_per_level: 0.43,
    class: "Mage"
};

static FREYA:God = God
{
    name: "Freya",
    base_health: 651.0,
    health_per_level: 84.0,
    base_mana: 257.0,
    mana_per_level: 37.0,
    base_as: 0.97,
    as_per_level: 0.0195,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.52,
    hp5_per_level: 0.52,
    base_mp5: 4.89,
    mp5_per_level: 0.39,
    class: "Mage"
};

static HADES:God = God
{
    name: "Hades",
    base_health: 754.0,
    health_per_level: 89.0,
    base_mana: 318.0,
    mana_per_level: 53.0,
    base_as: 0.96,
    as_per_level: 0.0105,
    base_auto_damage: 34.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 5.54,
    hp5_per_level: 0.54,
    base_mp5: 5.36,
    mp5_per_level: 0.36,
    class: "Mage"
};

static HEBO:God = God
{
    name: "He Bo",
    base_health: 630.0,
    health_per_level: 70.0,
    base_mana: 354.0,
    mana_per_level: 56.0,
    base_as: 0.87,
    as_per_level: 0.009,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 15.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.3,
    mp5_per_level: 0.4,
    class: "Mage"
};

static HEL:God = God
{
    name: "Hel",
    base_health: 644.0,
    health_per_level: 70.0,
    base_mana: 357.0,
    mana_per_level: 57.0,
    base_as: 0.87,
    as_per_level: 0.009,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 15.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 5.43,
    hp5_per_level: 0.43,
    base_mp5: 5.45,
    mp5_per_level: 0.45,
    class: "Mage"
};

static HERA:God = God
{
    name: "Hera",
    base_health: 603.0,
    health_per_level: 71.0,
    base_mana: 310.0,
    mana_per_level: 45.0,
    base_as: 0.91,
    as_per_level: 0.01,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.21,
    mp5_per_level: 0.41,
    class: "Mage"
};

static IX:God = God
{
    name: "Ix Chel",
    base_health: 616.0,
    health_per_level: 84.0,
    base_mana: 344.0,
    mana_per_level: 44.0,
    base_as: 0.91,
    as_per_level: 0.008,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.6,
    hp5_per_level: 0.6,
    base_mp5: 5.45,
    mp5_per_level: 0.45,
    class: "Mage"
};

static JANUS:God = God
{
    name: "Janus",
    base_health: 634.0,
    health_per_level: 74.0,
    base_mana: 408.0,
    mana_per_level: 58.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 17.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.45,
    mp5_per_level: 0.45,
    class: "Mage"
};

static KUKU:God = God
{
    name: "Kukulkan",
    base_health: 611.0,
    health_per_level: 79.0,
    base_mana: 310.0,
    mana_per_level: 45.0,
    base_as: 0.88,
    as_per_level: 0.091,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.46,
    hp5_per_level: 0.46,
    base_mp5: 5.45,
    mp5_per_level: 0.45,
    class: "Mage"
};

static MERLIN:God = God
{
    name: "Merlin",
    base_health: 597.0,
    health_per_level: 79.0,
    base_mana: 305.0,
    mana_per_level: 55.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 17.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.48,
    hp5_per_level: 0.48,
    base_mp5: 5.24,
    mp5_per_level: 0.44,
    class: "Mage"
};

static MORGANA:God = God
{
    name: "Morgan Le Fay",
    base_health: 683.0,
    health_per_level: 81.0,
    base_mana: 305.0,
    mana_per_level: 55.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.48,
    hp5_per_level: 0.48,
    base_mp5: 5.24,
    mp5_per_level: 0.44,
    class: "Mage"
};

static NOX:God = God
{
    name: "Nox",
    base_health: 590.0,
    health_per_level: 79.0,
    base_mana: 228.0,
    mana_per_level: 38.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.6,
    hp5_per_level: 0.6,
    base_mp5: 5.4,
    mp5_per_level: 0.4,
    class: "Mage"
};

static NUWA:God = God
{
    name: "Nu Wa",
    base_health: 639.0,
    health_per_level: 75.0,
    base_mana: 308.0,
    mana_per_level: 43.0,
    base_as: 1.02,
    as_per_level: 0.017,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.45,
    hp5_per_level: 0.45,
    base_mp5: 5.03,
    mp5_per_level: 0.43,
    class: "Mage"
};

static OLORUN:God = God
{
    name: "Olorun",
    base_health: 709.0,
    health_per_level: 79.0,
    base_mana: 298.0,
    mana_per_level: 58.0,
    base_as: 1.07,
    as_per_level: 0.015,
    base_auto_damage: 42.0,
    auto_damage_per_level: 2.38,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.45,
    hp5_per_level: 0.45,
    base_mp5: 5.4,
    mp5_per_level: 0.4,
    class: "Mage"
};

static PERSE:God = God
{
    name: "Persephone",
    base_health: 597.0,
    health_per_level: 79.0,
    base_mana: 335.0,
    mana_per_level: 55.0,
    base_as: 0.96,
    as_per_level: 0.095,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.7,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.45,
    hp5_per_level: 0.45,
    base_mp5: 5.42,
    mp5_per_level: 0.42,
    class: "Mage"
};

static POSEIDON:God = God
{
    name: "Poseidon",
    base_health: 602.0,
    health_per_level: 70.0,
    base_mana: 285.0,
    mana_per_level: 40.0,
    base_as: 0.88,
    as_per_level: 0.012,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 14.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.5,
    hp5_per_level: 0.5,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Mage"
};

static RA:God = God
{
    name: "Ra",
    base_health: 610.0,
    health_per_level: 71.0,
    base_mana: 303.0,
    mana_per_level: 48.0,
    base_as: 0.89,
    as_per_level: 0.01,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 17.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.48,
    hp5_per_level: 0.48,
    base_mp5: 5.24,
    mp5_per_level: 0.44,
    class: "Mage"
};

static RAIJIN:God = God
{
    name: "Raijin",
    base_health: 623.0,
    health_per_level: 84.0,
    base_mana: 298.0,
    mana_per_level: 43.0,
    base_as: 0.88,
    as_per_level: 0.009,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 16.0,
    phys_prots_per_level: 3.3,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.06,
    mp5_per_level: 0.46,
    class: "Mage"
};

static SCYLLA:God = God
{
    name: "Scylla",
    base_health: 590.0,
    health_per_level: 79.0,
    base_mana: 354.0,
    mana_per_level: 56.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 15.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.3,
    mp5_per_level: 0.4,
    class: "Mage"
};

static SOL:God = God
{
    name: "Sol",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 357.0,
    mana_per_level: 57.0,
    base_as: 1.02,
    as_per_level: 0.018,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.45,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 15.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.3,
    mp5_per_level: 0.4,
    class: "Mage"
};

static MORRI:God = God
{
    name: "The Morrigan",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 288.0,
    mana_per_level: 38.0,
    base_as: 0.88,
    as_per_level: 0.01,
    base_auto_damage: 36.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.7,
    hp5_per_level: 0.7,
    base_mp5: 4.9,
    mp5_per_level: 0.4,
    class: "Mage"
};

static THOTH:God = God
{
    name: "Thoth",
    base_health: 590.0,
    health_per_level: 79.0,
    base_mana: 294.0,
    mana_per_level: 49.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 34.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 15.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.04,
    mp5_per_level: 0.44,
    class: "Mage"
};

static TIAMAT:God = God
{
    name: "Tiamat",
    base_health: 641.0,
    health_per_level: 81.0,
    base_mana: 355.0,
    mana_per_level: 55.0,
    base_as: 0.89,
    as_per_level: 0.01,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.5,
    hp5_per_level: 0.5,
    base_mp5: 5.44,
    mp5_per_level: 0.44,
    class: "Mage"
};

static VULCAN:God = God
{
    name: "Vulcan",
    base_health: 607.0,
    health_per_level: 75.0,
    base_mana: 285.0,
    mana_per_level: 40.0,
    base_as: 0.91,
    as_per_level: 0.011,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.48,
    hp5_per_level: 0.48,
    base_mp5: 5.25,
    mp5_per_level: 0.45,
    class: "Mage"
};

static YUHUANG:God = God
{
    name: "Yu Huang",
    base_health: 641.0,
    health_per_level: 81.0,
    base_mana: 335.0,
    mana_per_level: 55.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.47,
    hp5_per_level: 0.47,
    base_mp5: 5.14,
    mp5_per_level: 0.44,
    class: "Mage"
};

static ZEUS:God = God
{
    name: "Zeus",
    base_health: 607.0,
    health_per_level: 75.0,
    base_mana: 289.0,
    mana_per_level: 44.0,
    base_as: 0.96,
    as_per_level: 0.099,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 14.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.45,
    hp5_per_level: 0.45,
    base_mp5: 5.05,
    mp5_per_level: 0.45,
    class: "Mage"
};

static ZHONG:God = God
{
    name: "Zhong Kui",
    base_health: 719.0,
    health_per_level: 89.0,
    base_mana: 297.0,
    mana_per_level: 47.0,
    base_as: 0.87,
    as_per_level: 0.01,
    base_auto_damage: 35.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 2.8,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.44,
    hp5_per_level: 0.44,
    base_mp5: 5.25,
    mp5_per_level: 0.45,
    class: "Mage"
};



//-------------------------------------------------------------------------
//Gods GUARDIANS
static ARES:God = God
{
    name: "Ares",
    base_health: 774.0,
    health_per_level: 95.0,
    base_mana: 237.0,
    mana_per_level: 37.0,
    base_as: 0.91,
    as_per_level: 0.0125,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.55,
    auto_progression: [1.0,0.75,1.0,1.25,0.0,0.0,0.0],
    base_phys_prots: 31.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.67,
    hp5_per_level: 0.67,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static ARTIO:God = God
{
    name: "Artio",
    base_health: 800.0,
    health_per_level: 100.0,
    base_mana: 249.0,
    mana_per_level: 39.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.9,
    hp5_per_level: 0.9,
    base_mp5: 5.03,
    mp5_per_level: 0.43,
    class: "Guardian"
};

static ATHENA:God = God
{
    name: "Athena",
    base_health: 805.0,
    health_per_level: 105.0,
    base_mana: 224.0,
    mana_per_level: 34.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,2.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.9,
    hp5_per_level: 0.9,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static ATLAS:God = God
{
    name: "Atlas",
    base_health: 819.0,
    health_per_level: 105.0,
    base_mana: 225.0,
    mana_per_level: 35.0,
    base_as: 0.96,
    as_per_level: 0.012,
    base_auto_damage: 39.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.25,1.25,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.4,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static BACCHUS:God = God
{
    name: "Bacchus",
    base_health: 783.0,
    health_per_level: 90.0,
    base_mana: 240.0,
    mana_per_level: 40.0,
    base_as: 0.89,
    as_per_level: 0.01,
    base_auto_damage: 39.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.2,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static CABRAKAN:God = God
{
    name: "Cabrakan",
    base_health: 781.0,
    health_per_level: 95.0,
    base_mana: 238.0,
    mana_per_level: 38.0,
    base_as: 0.91,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.6,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.75,
    hp5_per_level: 0.75,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static CERBERUS:God = God
{
    name: "Cerberus",
    base_health: 786.0,
    health_per_level: 100.0,
    base_mana: 235.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.01,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,0.8,0.8,0.0,0.0,0.0,0.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.75,
    hp5_per_level: 0.75,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static CTHULHU:God = God
{
    name: "Cthulhu",
    base_health: 800.0,
    health_per_level: 100.0,
    base_mana: 248.0,
    mana_per_level: 38.0,
    base_as: 0.91,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 33.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static FAFNIR:God = God
{
    name: "Fafnir",
    base_health: 788.0,
    health_per_level: 95.0,
    base_mana: 240.0,
    mana_per_level: 40.0,
    base_as: 0.91,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 33.0,
    phys_prots_per_level: 3.7,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.54,
    hp5_per_level: 0.54,
    base_mp5: 4.95,
    mp5_per_level: 0.45,
    class: "Guardian"
};

static GANESHA:God = God
{
    name: "Ganesha",
    base_health: 793.0,
    health_per_level: 100.0,
    base_mana: 240.0,
    mana_per_level: 40.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.55,
    auto_progression: [1.05,0.8,0.8,1.5,0.95,0.0,0.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.67,
    hp5_per_level: 0.67,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static GEB:God = God
{
    name: "Geb",
    base_health: 809.0,
    health_per_level: 95.0,
    base_mana: 224.0,
    mana_per_level: 34.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static JORM:God = God
{
    name: "Jormungandr",
    base_health: 798.0,
    health_per_level: 105.0,
    base_mana: 230.0,
    mana_per_level: 40.0,
    base_as: 1.01,
    as_per_level: 0.01,
    base_auto_damage: 30.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 33.0,
    phys_prots_per_level: 3.8,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.9,
    hp5_per_level: 0.9,
    base_mp5: 4.95,
    mp5_per_level: 0.45,
    class: "Guardian"
};

static KHEPRI:God = God
{
    name: "Khepri",
    base_health: 781.0,
    health_per_level: 95.0,
    base_mana: 224.0,
    mana_per_level: 34.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,0.5,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static KUMBHAKARNA:God = God
{
    name: "Kumbhakarna",
    base_health: 791.0,
    health_per_level: 105.0,
    base_mana: 234.0,
    mana_per_level: 34.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 29.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.75,
    hp5_per_level: 0.75,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static KUZENBO:God = God
{
    name: "Kuzenbo",
    base_health: 805.0,
    health_per_level: 105.0,
    base_mana: 234.0,
    mana_per_level: 34.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 29.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.85,
    hp5_per_level: 0.85,
    base_mp5: 5.02,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static MAUI:God = God
{
    name: "Maui",
    base_health: 793.0,
    health_per_level: 100.0,
    base_mana: 236.0,
    mana_per_level: 36.0,
    base_as: 0.86,
    as_per_level: 0.012,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 33.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 4.92,
    mp5_per_level: 0.42,
    class: "Guardian"
};

static SOBEK:God = God
{
    name: "Sobek",
    base_health: 786.0,
    health_per_level: 100.0,
    base_mana: 245.0,
    mana_per_level: 35.0,
    base_as: 0.86,
    as_per_level: 0.014,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.75,
    hp5_per_level: 0.75,
    base_mp5: 4.93,
    mp5_per_level: 0.43,
    class: "Guardian"
};

static SYLVANUS:God = God
{
    name: "Sylvanus",
    base_health: 809.0,
    health_per_level: 95.0,
    base_mana: 224.0,
    mana_per_level: 34.0,
    base_as: 0.91,
    as_per_level: 0.008,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static TERRA:God = God
{
    name: "Terra",
    base_health: 800.0,
    health_per_level: 100.0,
    base_mana: 235.0,
    mana_per_level: 35.0,
    base_as: 0.91,
    as_per_level: 0.014,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 32.0,
    phys_prots_per_level: 3.5,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Guardian"
};

static XING:God = God
{
    name: "Xing Tian",
    base_health: 788.0,
    health_per_level: 95.0,
    base_mana: 240.0,
    mana_per_level: 40.0,
    base_as: 0.91,
    as_per_level: 0.012,
    base_auto_damage: 37.0,
    auto_damage_per_level: 1.55,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 33.0,
    phys_prots_per_level: 3.7,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 5.54,
    hp5_per_level: 0.54,
    base_mp5: 4.95,
    mp5_per_level: 0.45,
    class: "Guardian"
};

static YEMOJA:God = God
{
    name: "Yemoja",
    base_health: 786.0,
    health_per_level: 100.0,
    base_mana: 0.0,
    mana_per_level: 0.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 36.0,
    auto_damage_per_level: 1.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 30.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.7,
    hp5_per_level: 0.7,
    base_mp5: 0.0,
    mp5_per_level: 0.0,
    class: "Guardian"
};

static YMIR:God = God
{
    name: "Ymir",
    base_health: 823.0,
    health_per_level: 109.0,
    base_mana: 232.0,
    mana_per_level: 32.0,
    base_as: 0.86,
    as_per_level: 0.012,
    base_auto_damage: 40.0,
    auto_damage_per_level: 1.55,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 36.0,
    phys_prots_per_level: 3.6,
    base_magical_prots: 36.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.9,
    hp5_per_level: 0.9,
    base_mp5: 4.9,
    mp5_per_level: 0.4,
    class: "Guardian"
};

//-------------------------------------------------------------------------
//Gods HUNTERS
static AMC:God = God
{
    name: "Ah Muzen Cab",
    base_health: 707.0,
    health_per_level: 77.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 0.97,
    as_per_level: 0.017,
    base_auto_damage: 35.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.71,
    hp5_per_level: 0.71,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static ANHUR:God = God
{
    name: "Anhur",
    base_health: 726.0,
    health_per_level: 82.0,
    base_mana: 255.0,
    mana_per_level: 35.0,
    base_as: 0.92,
    as_per_level: 0.017,
    base_auto_damage: 37.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.69,
    hp5_per_level: 0.69,
    base_mp5: 4.82,
    mp5_per_level: 0.32,
    class: "Hunter"
};

static APOLLO:God = God
{
    name: "Apollo",
    base_health: 711.0,
    health_per_level: 81.0,
    base_mana: 265.0,
    mana_per_level: 40.0,
    base_as: 0.97,
    as_per_level: 0.017,
    base_auto_damage: 37.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 2.9,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.69,
    hp5_per_level: 0.69,
    base_mp5: 5.0,
    mp5_per_level: 0.4,
    class: "Hunter"
};

static ARTEMIS:God = God
{
    name: "Artemis",
    base_health: 709.0,
    health_per_level: 79.0,
    base_mana: 239.0,
    mana_per_level: 34.0,
    base_as: 0.97,
    as_per_level: 0.017,
    base_auto_damage: 33.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.68,
    hp5_per_level: 0.68,
    base_mp5: 4.65,
    mp5_per_level: 0.25,
    class: "Hunter"
};

static CERN:God = God
{
    name: "Cernunnos",
    base_health: 756.0,
    health_per_level: 84.0,
    base_mana: 257.0,
    mana_per_level: 37.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 35.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.69,
    hp5_per_level: 0.69,
    base_mp5: 4.82,
    mp5_per_level: 0.32,
    class: "Hunter"
};

static CHARYBDIS:God = God
{
    name: "Charybdis",
    base_health: 697.0,
    health_per_level: 81.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 0.97,
    as_per_level: 0.016,
    base_auto_damage: 36.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,0.333,0.333,0.333,0.0,0.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.69,
    hp5_per_level: 0.69,
    base_mp5: 4.78,
    mp5_per_level: 0.32,
    class: "Hunter"
};

static CHERNO:God = God
{
    name: "Chernobog",
    base_health: 714.0,
    health_per_level: 84.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 0.97,
    as_per_level: 0.017,
    base_auto_damage: 34.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 19.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.67,
    hp5_per_level: 0.67,
    base_mp5: 4.75,
    mp5_per_level: 0.35,
    class: "Hunter"
};

static CHIRON:God = God
{
    name: "Chiron",
    base_health: 723.0,
    health_per_level: 79.0,
    base_mana: 265.0,
    mana_per_level: 40.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 32.0,
    auto_damage_per_level: 2.35,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.65,
    hp5_per_level: 0.65,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static CUPID:God = God
{
    name: "Cupid",
    base_health: 699.0,
    health_per_level: 76.0,
    base_mana: 269.0,
    mana_per_level: 39.0,
    base_as: 0.97,
    as_per_level: 0.018,
    base_auto_damage: 32.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 19.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.67,
    hp5_per_level: 0.67,
    base_mp5: 4.95,
    mp5_per_level: 0.35,
    class: "Hunter"
};

static DANZA:God = God
{
    name: "Danzaburou",
    base_health: 726.0,
    health_per_level: 82.0,
    base_mana: 268.0,
    mana_per_level: 38.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 38.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.69,
    hp5_per_level: 0.69,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static HACHI:God = God
{
    name: "Hachiman",
    base_health: 718.0,
    health_per_level: 81.0,
    base_mana: 266.0,
    mana_per_level: 31.0,
    base_as: 1.01,
    as_per_level: 0.013,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.69,
    hp5_per_level: 0.69,
    base_mp5: 4.87,
    mp5_per_level: 0.37,
    class: "Hunter"
};

static HEIM:God = God
{
    name: "Heimdallr",
    base_health: 756.0,
    health_per_level: 84.0,
    base_mana: 264.0,
    mana_per_level: 34.0,
    base_as: 0.97,
    as_per_level: 0.015,
    base_auto_damage: 30.0,
    auto_damage_per_level: 3.0,
    auto_progression: [1.25,1.25,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 26.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 4.9,
    mp5_per_level: 0.4,
    class: "Hunter"
};

static HOUYI:God = God
{
    name: "Hou Yi",
    base_health: 754.0,
    health_per_level: 82.0,
    base_mana: 278.0,
    mana_per_level: 38.0,
    base_as: 0.96,
    as_per_level: 0.012,
    base_auto_damage: 38.0,
    auto_damage_per_level: 2.8,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.68,
    hp5_per_level: 0.68,
    base_mp5: 5.35,
    mp5_per_level: 0.35,
    class: "Hunter"
};

static ISHTAR:God = God
{
    name: "Ishtar",
    base_health: 740.0,
    health_per_level: 82.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.016,
    base_auto_damage: 34.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.13,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.7,
    hp5_per_level: 0.7,
    base_mp5: 4.88,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static IZANAMI:God = God
{
    name: "Izanami",
    base_health: 711.0,
    health_per_level: 81.0,
    base_mana: 245.0,
    mana_per_level: 35.0,
    base_as: 0.94,
    as_per_level: 0.017,
    base_auto_damage: 31.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.1,1.1,1.1,1.1,1.1,1.1,1.1],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.7,
    hp5_per_level: 0.7,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static JING:God = God
{
    name: "Jing Wei",
    base_health: 705.0,
    health_per_level: 82.0,
    base_mana: 241.0,
    mana_per_level: 36.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 36.0,
    auto_damage_per_level: 2.7,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.68,
    hp5_per_level: 0.69,
    base_mp5: 4.7,
    mp5_per_level: 0.3,
    class: "Hunter"
};

static MANTI:God = God //stats are currently PH
{
    name: "Martichoras",
    base_health: 718.0,
    health_per_level: 81.0,
    base_mana: 266.0,
    mana_per_level: 31.0,
    base_as: 1.01,
    as_per_level: 0.013,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.69,
    hp5_per_level: 0.69,
    base_mp5: 4.87,
    mp5_per_level: 0.37,
    class: "Hunter"
};

static MEDUSA:God = God
{
    name: "Medusa",
    base_health: 739.0,
    health_per_level: 81.0,
    base_mana: 254.0,
    mana_per_level: 34.0,
    base_as: 0.96,
    as_per_level: 0.014,
    base_auto_damage: 36.0,
    auto_damage_per_level: 2.6,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.68,
    hp5_per_level: 0.68,
    base_mp5: 4.65,
    mp5_per_level: 0.25,
    class: "Hunter"
};

static NEITH:God = God
{
    name: "Neith",
    base_health: 719.0,
    health_per_level: 79.0,
    base_mana: 269.0,
    mana_per_level: 39.0,
    base_as: 1.02,
    as_per_level: 0.016,
    base_auto_damage: 36.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.54,
    hp5_per_level: 0.54,
    base_mp5: 4.75,
    mp5_per_level: 0.35,
    class: "Hunter"
};

static RAMA:God = God
{
    name: "Rama",
    base_health: 724.0,
    health_per_level: 80.0,
    base_mana: 239.0,
    mana_per_level: 34.0,
    base_as: 0.97,
    as_per_level: 0.017,
    base_auto_damage: 38.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.65,
    hp5_per_level: 0.65,
    base_mp5: 4.75,
    mp5_per_level: 0.25,
    class: "Hunter"
};

static SKADI:God = God
{
    name: "Skadi",
    base_health: 740.0,
    health_per_level: 82.0,
    base_mana: 255.0,
    mana_per_level: 35.0,
    base_as: 0.97,
    as_per_level: 0.016,
    base_auto_damage: 33.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.69,
    hp5_per_level: 0.69,
    base_mp5: 4.82,
    mp5_per_level: 0.32,
    class: "Hunter"
};

static ULLR:God = God
{
    name: "Ullr",
    base_health: 756.0,
    health_per_level: 84.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 0.96,
    as_per_level: 0.015,
    base_auto_damage: 35.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.71,
    hp5_per_level: 0.71,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Hunter"
};

static XBAL:God = God
{
    name: "Xbalanque",
    base_health: 716.0,
    health_per_level: 79.0,
    base_mana: 257.0,
    mana_per_level: 37.0,
    base_as: 0.96,
    as_per_level: 0.015,
    base_auto_damage: 38.0,
    auto_damage_per_level: 2.5,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.68,
    hp5_per_level: 0.68,
    base_mp5: 4.8,
    mp5_per_level: 0.4,
    class: "Hunter"
};

//-------------------------------------------------------------------------
//Gods ASSASSINS
static ARACHNE:God = God
{
    name: "Arachne",
    base_health: 706.0,
    health_per_level: 83.0,
    base_mana: 251.0,
    mana_per_level: 41.0,
    base_as: 1.02,
    as_per_level: 0.02,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.2,
    auto_progression: [0.5,1.0,1.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.67,
    hp5_per_level: 0.67,
    base_mp5: 5.26,
    mp5_per_level: 0.46,
    class: "Assassin"
};

static AWILIX:God = God
{
    name: "Awilix",
    base_health: 746.0,
    health_per_level: 81.0,
    base_mana: 278.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.0189,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.16,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.68,
    hp5_per_level: 0.68,
    base_mp5: 4.36,
    mp5_per_level: 0.46,
    class: "Assassin"
};

static BAKA:God = God
{
    name: "Bakasura",
    base_health: 719.0,
    health_per_level: 82.0,
    base_mana: 239.0,
    mana_per_level: 39.0,
    base_as: 1.02,
    as_per_level: 0.016,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.68,
    hp5_per_level: 0.68,
    base_mp5: 5.26,
    mp5_per_level: 0.46,
    class: "Assassin"
};

static BASTET:God = God
{
    name: "Bastet",
    base_health: 665.0,
    health_per_level: 84.0,
    base_mana: 269.0,
    mana_per_level: 39.0,
    base_as: 1.02,
    as_per_level: 0.0198,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.21,
    auto_progression: [1.0,0.7,1.3,0.0,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.72,
    hp5_per_level: 0.72,
    base_mp5: 4.65,
    mp5_per_level: 0.25,
    class: "Assassin"
};

static CAMA:God = God
{
    name: "Camazotz",
    base_health: 698.0,
    health_per_level: 82.0,
    base_mana: 275.0,
    mana_per_level: 35.0,
    base_as: 1.02,
    as_per_level: 0.02,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 19.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.67,
    hp5_per_level: 0.67,
    base_mp5: 4.7,
    mp5_per_level: 0.3,
    class: "Assassin"
};

static CLIO:God = God
{
    name: "Cliodhna",
    base_health: 746.0,
    health_per_level: 81.0,
    base_mana: 265.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.0216,
    base_auto_damage: 43.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,1.0,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.3,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.85,
    hp5_per_level: 0.85,
    base_mp5: 4.7,
    mp5_per_level: 0.4,
    class: "Assassin"
};

static DAJI:God = God
{
    name: "Da Ji",
    base_health: 696.0,
    health_per_level: 80.0,
    base_mana: 250.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.02,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 4.97,
    mp5_per_level: 0.37,
    class: "Assassin"
};

static FENRIR:God = God
{
    name: "Fenrir",
    base_health: 719.0,
    health_per_level: 82.0,
    base_mana: 265.0,
    mana_per_level: 35.0,
    base_as: 1.02,
    as_per_level: 0.017,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.73,
    hp5_per_level: 0.73,
    base_mp5: 4.46,
    mp5_per_level: 0.26,
    class: "Assassin"
};

static HUNBATZ:God = God
{
    name: "Hun Batz",
    base_health: 723.0,
    health_per_level: 79.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.0189,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.16,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.68,
    hp5_per_level: 0.68,
    base_mp5: 4.17,
    mp5_per_level: 0.27,
    class: "Assassin"
};

static KALI:God = God
{
    name: "Kali",
    base_health: 736.0,
    health_per_level: 78.0,
    base_mana: 259.0,
    mana_per_level: 34.0,
    base_as: 1.02,
    as_per_level: 0.0216,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.32,
    auto_progression: [1.0,0.5,0.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 24.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.68,
    hp5_per_level: 0.68,
    base_mp5: 4.51,
    mp5_per_level: 0.21,
    class: "Assassin"
};

static LANCELOT:God = God
{
    name: "Lancelot",
    base_health: 725.0,
    health_per_level: 81.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.019,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,1.0,1.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 2.06,
    mp5_per_level: 0.46,
    class: "Assassin"
};

static LOKI:God = God
{
    name: "Loki",
    base_health: 632.0,
    health_per_level: 79.0,
    base_mana: 245.0,
    mana_per_level: 35.0,
    base_as: 1.02,
    as_per_level: 0.019,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,0.5,0.5,0.5,1.5,0.0,0.0],
    base_phys_prots: 18.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.7,
    hp5_per_level: 0.7,
    base_mp5: 4.55,
    mp5_per_level: 0.35,
    class: "Assassin"
};

static MERC:God = God
{
    name: "Mercury",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 240.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.024,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.13,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.7,
    hp5_per_level: 0.7,
    base_mp5: 4.75,
    mp5_per_level: 0.35,
    class: "Assassin"
};

static NEZHA:God = God
{
    name: "Ne zha",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 235.0,
    mana_per_level: 35.0,
    base_as: 1.02,
    as_per_level: 0.021,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,1.0,1.5,2.0,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.71,
    hp5_per_level: 0.71,
    base_mp5: 4.17,
    mp5_per_level: 0.27,
    class: "Assassin"
};

static NEMESIS:God = God
{
    name: "Nemesis",
    base_health: 738.0,
    health_per_level: 80.0,
    base_mana: 248.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.0216,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,1.0,0.75,1.25,0.0,0.0,0.0],
    base_phys_prots: 19.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.7,
    hp5_per_level: 0.7,
    base_mp5: 4.7,
    mp5_per_level: 0.4,
    class: "Assassin"
};

static PELE:God = God
{
    name: "Pele",
    base_health: 725.0,
    health_per_level: 81.0,
    base_mana: 280.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.02,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 4.7,
    mp5_per_level: 0.4,
    class: "Assassin"
};

static RAT:God = God
{
    name: "Ratatoskr",
    base_health: 665.0,
    health_per_level: 77.0,
    base_mana: 281.0,
    mana_per_level: 41.0,
    base_as: 1.02,
    as_per_level: 0.02,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,0.5,0.5,1.25,0.0,0.0,0.0],
    base_phys_prots: 20.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 5.06,
    mp5_per_level: 0.46,
    class: "Assassin"
};

static RAVANA:God = God
{
    name: "Ravana",
    base_health: 639.0,
    health_per_level: 79.0,
    base_mana: 267.0,
    mana_per_level: 37.0,
    base_as: 1.02,
    as_per_level: 0.017,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.2,
    auto_progression: [0.75,0.5,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.7,
    hp5_per_level: 0.7,
    base_mp5: 4.75,
    mp5_per_level: 0.35,
    class: "Assassin"
};

static SERQET:God = God
{
    name: "Serqet",
    base_health: 638.0,
    health_per_level: 78.0,
    base_mana: 280.0,
    mana_per_level: 40.0,
    base_as: 1.02,
    as_per_level: 0.0216,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 4.59,
    mp5_per_level: 0.29,
    class: "Assassin"
};

static SET:God = God
{
    name: "Set",
    base_health: 688.0,
    health_per_level: 79.0,
    base_mana: 248.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.019,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 4.58,
    mp5_per_level: 0.38,
    class: "Assassin"
};

static SUSANO:God = God
{
    name: "Susano",
    base_health: 689.0,
    health_per_level: 80.0,
    base_mana: 263.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.0216,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,1.0,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 4.47,
    mp5_per_level: 0.27,
    class: "Assassin"
};

static THANA:God = God
{
    name: "Thanatos",
    base_health: 625.0,
    health_per_level: 79.0,
    base_mana: 278.0,
    mana_per_level: 38.0,
    base_as: 1.02,
    as_per_level: 0.017,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,0.75,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.67,
    hp5_per_level: 0.67,
    base_mp5: 4.78,
    mp5_per_level: 0.38,
    class: "Assassin"
};

static THOR:God = God
{
    name: "Thor",
    base_health: 745.0,
    health_per_level: 80.0,
    base_mana: 278.0,
    mana_per_level: 38.0,
    base_as: 1.01,
    as_per_level: 0.0145,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 23.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.69,
    hp5_per_level: 0.69,
    base_mp5: 4.69,
    mp5_per_level: 0.29,
    class: "Assassin"
};

static TSUKU:God = God
{
    name: "Tsukuyomi",
    base_health: 719.0,
    health_per_level: 82.0,
    base_mana: 240.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.013,
    base_auto_damage: 37.0,
    auto_damage_per_level: 2.4,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 21.0,
    phys_prots_per_level: 3.0,
    base_magical_prots: 31.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.68,
    hp5_per_level: 0.68,
    base_mp5: 4.96,
    mp5_per_level: 0.46,
    class: "Assassin"
};

//-------------------------------------------------------------------------
//Gods WARRIORS
static ACHILLES:God = God
{
    name: "Achilles",
    base_health: 754.0,
    health_per_level: 89.0,
    base_mana: 240.0,
    mana_per_level: 35.0,
    base_as: 0.96,
    as_per_level: 0.0125,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,1.0,1.0,1.0,1.0,1.0,1.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.75,
    hp5_per_level: 0.75,
    base_mp5: 5.09,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static AMATERASU:God = God
{
    name: "Amaterasu",
    base_health: 761.0,
    health_per_level: 89.0,
    base_mana: 255.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.5,1.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.2,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static BELLONA:God = God
{
    name: "Bellona",
    base_health: 778.0,
    health_per_level: 92.0,
    base_mana: 255.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 38.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.0,0.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 5.2,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static CHAAC:God = God
{
    name: "Chaac",
    base_health: 778.0,
    health_per_level: 92.0,
    base_mana: 240.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.1,
    auto_progression: [1.0,0.5,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.8,
    hp5_per_level: 0.8,
    base_mp5: 5.09,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static CU_CHU:God = God
{
    name: "Cu Chulainn",
    base_health: 761.0,
    health_per_level: 89.0,
    base_mana: 100.0,
    mana_per_level: 0.0,
    base_as: 1.01,
    as_per_level: 0.0125,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [0.7,0.7,1.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.75,
    hp5_per_level: 0.75,
    base_mp5: 0.0,
    mp5_per_level: 0.0,
    class: "Warrior"
};

static ERLANG:God = God
{
    name: "Erlang Shen",
    base_health: 768.0,
    health_per_level: 89.0,
    base_mana: 255.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.01,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [0.75,0.75,0.75,1.1,0.9,0.0,0.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.6,
    hp5_per_level: 0.6,
    base_mp5: 5.0,
    mp5_per_level: 0.3,
    class: "Warrior"
};

static GILG:God = God
{
    name: "Gilgamesh",
    base_health: 761.0,
    health_per_level: 89.0,
    base_mana: 245.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.0125,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.3,
    auto_progression: [1.0,0.0,0.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.8,
    hp5_per_level: 0.8,
    base_mp5: 4.69,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static GUAN:God = God
{
    name: "Guan Yu",
    base_health: 763.0,
    health_per_level: 91.0,
    base_mana: 259.0,
    mana_per_level: 39.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,1.0,1.3,1.75,0.0,0.0,0.0],
    base_phys_prots: 26.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.7,
    hp5_per_level: 0.7,
    base_mp5: 5.25,
    mp5_per_level: 0.45,
    class: "Warrior"
};

static HERCULES:God = God
{
    name: "Hercules",
    base_health: 779.0,
    health_per_level: 93.0,
    base_mana: 237.0,
    mana_per_level: 32.0,
    base_as: 1.01,
    as_per_level: 0.008,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,1.0,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 7.63,
    hp5_per_level: 0.63,
    base_mp5: 4.7,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static HORUS:God = God
{
    name: "Horus",
    base_health: 767.0,
    health_per_level: 88.0,
    base_mana: 267.0,
    mana_per_level: 37.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.1,
    auto_progression: [1.0,0.0,0.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.8,
    hp5_per_level: 0.8,
    base_mp5: 5.15,
    mp5_per_level: 0.45,
    class: "Warrior"
};

static ARTHUR:God = God
{
    name: "King Arthur",
    base_health: 766.0,
    health_per_level: 87.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 1.00,
    as_per_level: 0.0,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.0,0.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.8,
    hp5_per_level: 0.8,
    base_mp5: 4.8,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static MULAN:God = God
{
    name: "Mulan",
    base_health: 774.0,
    health_per_level: 88.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 1.01,
    as_per_level: 0.01,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,1.0,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.7,
    hp5_per_level: 0.7,
    base_mp5: 5.1,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static NIKE:God = God
{
    name: "Nike",
    base_health: 759.0,
    health_per_level: 87.0,
    base_mana: 258.0,
    mana_per_level: 38.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,1.0,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 24.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 10.8,
    hp5_per_level: 0.8,
    base_mp5: 4.99,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static ODIN:God = God
{
    name: "Odin",
    base_health: 745.0,
    health_per_level: 87.0,
    base_mana: 235.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.011,
    base_auto_damage: 40.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,1.0,1.0,1.5,0.0,0.0,0.0],
    base_phys_prots: 26.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.63,
    hp5_per_level: 0.63,
    base_mp5: 4.5,
    mp5_per_level: 0.4,
    class: "Warrior"
};

static OSIRIS:God = God
{
    name: "Osiris",
    base_health: 771.0,
    health_per_level: 92.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 1.01,
    as_per_level: 0.014,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.25,
    auto_progression: [0.5,1.0,0.5,1.0,0.0,0.0,0.0],
    base_phys_prots: 27.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 6.8,
    hp5_per_level: 0.8,
    base_mp5: 5.09,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static SHIVA:God = God
{
    name: "Shiva",
    base_health: 752.0,
    health_per_level: 87.0,
    base_mana: 246.0,
    mana_per_level: 36.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.75,1.25,0.0,0.0,0.0,0.0],
    base_phys_prots: 26.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.7,
    hp5_per_level: 0.7,
    base_mp5: 4.89,
    mp5_per_level: 0.39,
    class: "Warrior"
};

static SURTR:God = God
{
    name: "Surtr",
    base_health: 758.0,
    health_per_level: 88.0,
    base_mana: 265.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.012,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.2,
    auto_progression: [1.0,0.75,1.0,1.5,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 9.8,
    hp5_per_level: 0.8,
    base_mp5: 4.98,
    mp5_per_level: 0.38,
    class: "Warrior"
};

static WUKONG:God = God
{
    name: "Sun Wukong",
    base_health: 761.0,
    health_per_level: 89.0,
    base_mana: 240.0,
    mana_per_level: 35.0,
    base_as: 1.01,
    as_per_level: 0.009,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.75,1.0,1.25,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.8,
    hp5_per_level: 0.8,
    base_mp5: 4.68,
    mp5_per_level: 0.38,
    class: "Warrior"
};

static TYR:God = God
{
    name: "Tyr",
    base_health: 766.0,
    health_per_level: 87.0,
    base_mana: 270.0,
    mana_per_level: 40.0,
    base_as: 1.01,
    as_per_level: 0.009,
    base_auto_damage: 41.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.5,1.5,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.2,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.72,
    hp5_per_level: 0.72,
    base_mp5: 4.69,
    mp5_per_level: 0.29,
    class: "Warrior"
};

static VAMANA:God = God
{
    name: "Vamana",
    base_health: 778.0,
    health_per_level: 92.0,
    base_mana: 239.0,
    mana_per_level: 39.0,
    base_as: 1.014,
    as_per_level: 0.014,
    base_auto_damage: 39.0,
    auto_damage_per_level: 2.0,
    auto_progression: [1.0,0.0,0.0,0.0,0.0,0.0,0.0],
    base_phys_prots: 28.0,
    phys_prots_per_level: 3.1,
    base_magical_prots: 33.6,
    magical_prots_per_level: 1.6,
    base_hp5: 8.5,
    hp5_per_level: 0.5,
    base_mp5: 5.11,
    mp5_per_level: 0.41,
    class: "Warrior"
};

//-------------------------------------------------------------------------
//Functions

fn array_to_menu_string(array:&[&Item]) -> String
{
    let mut string = String::new();
    string.push_str("|");
    for i in array.iter()
    {
        string.push_str(i.name);
        string.push_str("|")
    }

    string.pop();
    string
}

fn string_to_god(string:&str) -> &God
{
    let mut output = &ERROR; 
    for i in GODS.iter()
    {
        if i.name == string
        {
            output = i;
        }
    }
    output
}

fn clamp(input:f32,min:f32,max:f32) -> f32
{
    if input < min
    {
        min
    }

    else if input > max
    {
        max
    }

    else
    {
        input
    }
}

fn combine_items(items:[&Item;6]) -> Build
{
    Build
    {
        names: [items[0].name,items[1].name,items[2].name,items[3].name,items[4].name,items[5].name],
        magical_power: items[0].magical_power + items[1].magical_power + items[2].magical_power + items[3].magical_power + items[4].magical_power + items[5].magical_power,
        physical_power: items[0].physical_power + items[1].physical_power + items[2].physical_power + items[3].physical_power + items[4].physical_power + items[5].physical_power,
        flat_pen: items[0].flat_pen + items[1].flat_pen + items[2].flat_pen + items[3].flat_pen + items[4].flat_pen + items[5].flat_pen,
        percent_pen: items[0].percent_pen + items[1].percent_pen + items[2].percent_pen + items[3].percent_pen + items[4].percent_pen + items[5].percent_pen,
        attack_speed: items[0].attack_speed + items[1].attack_speed + items[2].attack_speed + items[3].attack_speed + items[4].attack_speed + items[5].attack_speed,
        crit_chance: items[0].crit_chance + items[1].crit_chance + items[2].crit_chance + items[3].crit_chance + items[4].crit_chance + items[5].crit_chance,
        lifesteal: items[0].lifesteal + items[1].lifesteal + items[2].lifesteal + items[3].lifesteal + items[4].lifesteal + items[5].lifesteal,
        cdr: items[0].cdr + items[1].cdr + items[2].cdr + items[3].cdr + items[4].cdr + items[5].cdr,
        mana: items[0].mana + items[1].mana + items[2].mana + items[3].mana + items[4].mana + items[5].mana,
        health: items[0].health + items[1].health + items[2].health + items[3].health + items[4].health + items[5].health,
        phys_prot: items[0].phys_prot + items[1].phys_prot + items[2].phys_prot + items[3].phys_prot + items[4].phys_prot + items[5].phys_prot,
        magical_prot: items[0].magical_prot + items[1].magical_prot + items[2].magical_prot + items[3].magical_prot + items[4].magical_prot + items[5].magical_prot,
        mp5: items[0].mp5 + items[1].mp5 + items[2].mp5 + items[3].mp5 + items[4].mp5 + items[5].mp5,
        hp5: items[0].hp5 + items[1].hp5 + items[2].hp5 + items[3].hp5 + items[4].hp5 + items[5].hp5,
        ccr: items[0].ccr + items[1].ccr + items[2].ccr + items[3].ccr + items[4].ccr + items[5].ccr,
        move_speed: items[0].move_speed + items[1].move_speed + items[2].move_speed + items[3].move_speed + items[4].move_speed + items[5].move_speed,
        gold: items[0].gold + items[1].gold + items[2].gold + items[3].gold + items[4].gold + items[5].gold,
    }
}

fn ability_damage (base_ability_damage:f32, ability_scaling:f32, god:&God, level: f32, build:&Build, target:&God, target_level:f32, 
    target_build:&Build,obby_enabled:bool, soul_gem_enabled:bool, poly_enabled:bool,estaff_enabled:bool,conduit_enabled:bool,
    archmages_enabled:bool,focus_enabled:bool,magus_enabled:bool,doom_enabled:bool,binding_enabled:bool,
    bancroft_enabled:bool,tahuti_enabled:bool,magical:bool,titans_bane_enabled:bool,arondight_enabled:bool,serrated_enabled:bool,runeforged_enabled:bool,
    cowl_enabled:bool,redstone_attack_speed_enabled:bool,bumbas_spear_enabled:bool,protector_enabled:bool,mhb_enabled:bool,
    ability_crit:bool,ability_true_damage:bool,hydras_enabled:bool,as_buff:f32,sol_passive:bool,
    flat_power_buff:f32,percent_power_buff:f32,red_buff:bool,purp_buff:bool,fire_giant:bool,
    enhanced_fire_giant:bool,p500_pot:bool,pen_pot:bool,mitigations:f32,calam_enabled:bool,claw_enabled:bool,myr_enabled:bool,number_of_abilities:i32,
    divine_enabled:bool,boomerang_enabled:bool) -> f32
{
    let mut power = clamp((build.magical_power+ ((god.class=="Mage")as i32 as f32) * (20.0 + level)) * magical as i32 as f32 + build.physical_power * !magical as i32 as f32 + flat_power_buff,0.0,900.0 * magical as i32 as f32 + 400.0 * !magical as i32 as f32);
    let flat_pen = clamp(build.flat_pen + ((god.class=="Assassin")as i32 as f32) * (2.0 + 0.4 * level),0.0,50.0);
    let mut percent_pen = clamp(build.percent_pen,0.0,0.32);
    let mut attack_speed = god.base_as + (god.as_per_level * (level - 1.0)) + build.attack_speed * (god.base_as - god.as_per_level);

    let mut reaver_bool = false;
    let mut soul_gem_bool = false;
    let mut poly_bool = false;
    let mut estaff_bool = false;
    let mut conduit_bool = false;
    let mut archmage_bool = false;
    let mut sacrificial_damage_multi = 0.0;
    let mut focus_damage_multi = 0.0;
    let mut magus_multi = 0.0;
    let mut binding_shred = 0.0;
    let mut typhon_bool = false;
    let mut bancroft_lifesteal = 0.0; //just used for any lifesteal gained through passives and not part of the build. Hasn't been renamed out of laziness
    let mut tahuti_scaling_multi = 1.0;
    let mut calam_bool = false;
    let mut claw_bool = false;

    let mut crusher_bool = false;
    let mut heartseeker_bool = false;
    let mut arondight_bool = false;
    let mut silverbranch_bool = false;
    let mut runeforged_multi = 0.0;
    let mut bluestone_pendant_bool = false;
    let mut bluestone_brooch_bool = false;
    let mut redstone_bool = false;
    let mut percent_power_increase = 0.0;
    let mut mhb_bool = false;
    let mut toxic_blade_bool = false; //used to use the pen on mhb
    let mut deathbringer_bonus = 0.0;
    let mut hydras_bool = false;
    let mut basic_attack_damage = 0.0;
    let mut fighters_multi = 0.0;
    let mut rangdas_bool = false;
    let mut myr_multi = 0.0;
    let mut divine_bool = false;
    let mut tablet_bool = false;
    let mut boomerang_crit = 0.0;


    if god.name == KUKU.name {power += (build.mana + (god.base_mana + god.mana_per_level * (level - 1.0))) * 0.04; }
    if god.name == FREYA.name { bancroft_lifesteal += 0.07 + 0.002 * level; }    
    if sol_passive { percent_power_increase += 0.25;}

    let mut levels_in_ult = 0.0;
    if level>=5.0 { levels_in_ult += 1.0;}
    if level>=9.0 { levels_in_ult += 1.0;}
    if level>=13.0 { levels_in_ult += 1.0;}
    if level>=17.0 { levels_in_ult += 1.0;}
    if level==20.0 { levels_in_ult += 1.0;}

    if god.name == SERQET.name && levels_in_ult != 0.0 { boomerang_crit += 0.05 + 0.05*levels_in_ult;}
    if god.name == JANUS.name && levels_in_ult != 0.0 { percent_pen =clamp(percent_pen + 0.06 + 0.02*levels_in_ult,0.0,0.32);}

    for name in build.names.iter()
    {
        if name == &TYPHONS.name { typhon_bool = true;}
        if name == &BOOK_OF_THOTH.name { power += build.mana * 0.08;}
        if obby_enabled && name == &OBBY.name { percent_pen = percent_pen + 0.08; }        
        if name == &REAVER.name { reaver_bool = true;}
        if soul_gem_enabled && name == &SOUL_GEM.name { soul_gem_bool = true;}
        if poly_enabled && name == &POLY.name { poly_bool = true; }
        if estaff_enabled && name == &ESTAFF.name { estaff_bool = true;}
        if conduit_enabled && name == &CONDUIT.name { conduit_bool = true;}
        if archmages_enabled && name == &ARCHMAGES.name { archmage_bool = true;}
        if name == &SACRIFICIAL.name { sacrificial_damage_multi = 0.15; }
        if focus_enabled && name == &FOCUS.name { focus_damage_multi = 0.15; }
        if magus_enabled && name == &MAGUS.name { magus_multi = 0.05; }
        if doom_enabled && name == &DOOM_ORB.name { power += 20.0; }
        if binding_enabled && name == &BINDING.name { binding_shred = level; }
        if bancroft_enabled && (name == &BANCROFTS.name || name == &NIMBLE_BANCROFTS.name || name == &BANCROFTS_CLAW.name) { power += 100.0; bancroft_lifesteal = 0.1; }
        if tahuti_enabled && (name == &TAHUTI.name || name == &CALAM_TAHUTI.name) { tahuti_scaling_multi = 1.2; }
        if claw_enabled && name == &BANCROFTS_CLAW.name { claw_bool = true; }
        if name == &FIGHTERS.name { fighters_multi = 0.06 + (0.0025 * level); }
        if name == &RANGDAS.name { rangdas_bool = true; }
        if myr_enabled && name == &MYRDDIN.name { myr_multi = 0.2; }

        if name == &CRUSHER.name { crusher_bool = true; }
        if name == &HEARTSEEKER.name { heartseeker_bool = true; }
        if titans_bane_enabled && name == &TITANS.name { percent_pen = percent_pen + 0.16; }
        if arondight_enabled && name == &ARONDIGHT.name { arondight_bool = true; }
        if name == &TRANS.name { power += (build.mana + (god.base_mana + god.mana_per_level * (level - 1.0))) * 0.03; }
        if name == &SILVERBRANCH.name { silverbranch_bool = true; }
        if serrated_enabled && name == &SERRATED.name { power += 30.0; }
        if runeforged_enabled && name == &RUNEFORGED.name { runeforged_multi = 0.1; }
        if cowl_enabled
        {
            if name== &LEATHER_COWL.name { attack_speed += 0.1 * (god.base_as - god.as_per_level);}
            if name== &HUNTERS_COWL.name { attack_speed += 0.2 * (god.base_as - god.as_per_level);}
        }
        if name == &BLUESTONE_PENDANT.name { bluestone_pendant_bool = true;}
        if name == &BLUESTONE_BROOCH.name { bluestone_brooch_bool = true;}
        if name == &REDSTONE.name 
        { 
            redstone_bool = true; 
            if redstone_attack_speed_enabled{ attack_speed += 0.5 * (god.base_as - god.as_per_level);}
        }
        if bumbas_spear_enabled && name == &BUMBAS_SPEAR.name { percent_power_increase += 0.1; }
        if protector_enabled && name == &PROTECTOR.name {  percent_power_increase += 0.15;}
        if name == &SPARTAN.name {  percent_power_increase += 0.1;}
        
        if mhb_enabled &&  name == &MHB.name { mhb_bool = true;}
        if name == &TOXICBLADE.name { toxic_blade_bool = true;}
        if name == &DEATHBRINGER.name { deathbringer_bonus = 0.3; }
        if hydras_enabled && name == &HYDRAS.name { hydras_bool = true;}

        if name == &GILDED.name { basic_attack_damage += 20.0; }
        if name == &DIAMOND.name { basic_attack_damage += 80.0; }
        if name == &ORNATE.name { basic_attack_damage += 70.0; }
        if name == &MANIKIN_SCEPTER.name { basic_attack_damage += 10.0; }
        if name == &MANIKINS_MACE.name { basic_attack_damage += 50.0; }

        if calam_enabled && name == &CALAM_TAHUTI.name { calam_bool = true; }
        if divine_enabled && name == &DIVINE.name { divine_bool = true; }
        if name == &TABLET.name {tablet_bool = true;}

        if boomerang_enabled && name == &BOOMERANG.name { boomerang_crit += 0.3; }


    }

    

    let mut item_mitigations = 0.0;
    for name in target_build.names.iter()
    {
        if name == &ONI_HUNTERS.name { item_mitigations += 0.9; }
    }
    if target.name == CABRAKAN.name { item_mitigations += 0.05; }

    attack_speed += as_buff * (god.base_as - god.as_per_level);

    if red_buff
    {
        percent_power_increase += 0.1;
        if magical{ power += 10.0; }
        else { power += 5.0; }
    }

 


    if purp_buff
    { 
        attack_speed += 0.1 * (god.base_as - god.as_per_level);
        if magical{ basic_attack_damage += 15.0; }
        else{ basic_attack_damage += 12.0; }
    }



    if fire_giant
    {
        if magical{ power += 70.0; }
        else { power += 50.0; }
    }

    if enhanced_fire_giant
    {
        if magical{ power += 100.0; }
        else { power += 65.0; }
    }

    if p500_pot
    {
        if magical{ power += 70.0; }
        else { power += 40.0; }
    }

    if pen_pot
    {
        percent_power_increase += 0.25;
        percent_pen = clamp(percent_pen + 0.08,0.0,0.32)
    }

    if typhon_bool { power += clamp(build.lifesteal + bancroft_lifesteal,0.0,0.4)*200.0;}

    if silverbranch_bool{ power += clamp(attack_speed - 2.5,0.0,9999.9) * 100.0;  }

    let mut rangdas_multi = 0.0;
    if rangdas_bool
    {
        if magical{ rangdas_multi = clamp(0.03 * ((power / 90.0) - (power % 90.0)/90.0), 0.0, 0.15); }
        else { rangdas_multi = clamp(0.03 * ((power / 55.0) - (power % 65.0)/65.0), 0.0, 0.15); }
    }

    

    //applies percent increases to power
    power *= 1.0 + percent_power_increase + percent_power_buff;

    
    let unmitigated_ability_damage = base_ability_damage + (power * ability_scaling * tahuti_scaling_multi);
    let crit_multi = 1.0 + clamp(build.crit_chance + boomerang_crit,0.0,1.0) * (0.75 + deathbringer_bonus) * ((0.0 * magical as i32 as f32) + (1.0 * !magical as i32 as f32));
    let olorun_crit_multi = 1.0 + clamp(power/1000.0,0.0,0.7) * 0.3 * (god.name==OLORUN.name) as i32 as f32 * (power>=150.0) as i32 as f32;
    let unmitigated_ability_damage_after_crit = clamp(unmitigated_ability_damage * crit_multi * olorun_crit_multi * ability_crit as i32 as f32,unmitigated_ability_damage,99999.9);

    let target_magical_prots = target.base_magical_prots + (target.magical_prots_per_level * (target_level - 1.0)) + target_build.magical_prot;
    let target_physical_prots = target.base_phys_prots + (target.phys_prots_per_level * (target_level - 1.0)) + target_build.phys_prot;
    let target_prots = target_magical_prots * magical as i32 as f32 + target_physical_prots * !magical as i32 as f32;
    let target_prots_after_shred = target_prots - binding_shred;
    let target_prots_after_pen = (target_prots_after_shred * (1.0 - percent_pen)) - flat_pen;

    let protections_multiplier = clamp(100.0/(100.0+target_prots_after_pen) + 10.0 * ability_true_damage as i32 as f32,0.0,1.0);

    let mut mhb_damage = 0.0;
    if mhb_bool
    {
        let unmitigated_mhb_damage = 0.2 * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health); 
        if !magical
        {            
            mhb_damage = unmitigated_mhb_damage * protections_multiplier;
        }
        else //for magical is a bit special since MHB does physical damage
        {
            let target_physical_prots_after_shred_and_pen = (target_physical_prots - binding_shred) - (TOXICBLADE.flat_pen * toxic_blade_bool as i32 as f32);
            let physical_protections_multiplier = clamp(100.0/(100.0+target_physical_prots_after_shred_and_pen),0.0,1.0);
            mhb_damage = unmitigated_mhb_damage * physical_protections_multiplier;
        }
    }

    let mut unmitigated_reaver_damage = 0.0;
    if reaver_bool
    {
        //y=(7/75,000)x - (1/6)  
        //y = percent of health, x = health
        let percent_health = clamp((7.0/75000.0)*(target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health) - (1.0/6.0),0.02,0.09);
        unmitigated_reaver_damage = percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health);
        if number_of_abilities > 1
        {
            unmitigated_reaver_damage += (number_of_abilities as f32-1.0) * percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health) * 0.5;
        }
    }

    let mut unmitigated_claw_damage = 0.0;
    if claw_bool
    {
        let percent_health = 0.005 * ((power / 75.0) - (power % 75.0)/75.0);
        unmitigated_claw_damage = percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health) * clamp(number_of_abilities as f32,1.0,3.0);
    }

    let mut unmitigated_heartseeker_damage = 0.0;
    if heartseeker_bool
    {
        let percent_health = clamp((1.0/3750.0)*power-(1.0/30.0),0.02,0.06);
        unmitigated_heartseeker_damage = percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health);
        if number_of_abilities > 1
        {
            unmitigated_heartseeker_damage += (number_of_abilities as f32-1.0) * percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health) * 0.75
        }
    }

    let mut unmitigated_soul_gem_damage = 0.0;
    if soul_gem_bool{ unmitigated_soul_gem_damage = power * 0.2 * (((number_of_abilities as f32-1.0)/4.0).floor()+1.0);}
    else { unmitigated_soul_gem_damage = power * 0.25 * (number_of_abilities as f32/4.0).floor();}

    let mut unmitigated_archmages_damage = 0.0;
    if archmage_bool{ unmitigated_archmages_damage = power * 0.3;}

    let mut unmitigated_calam_damage = 0.0;
    if calam_bool{ unmitigated_calam_damage = 100.0 + (power * 0.35)}

    let mut unmitigated_divine_damage = 0.0;
    if divine_bool{ unmitigated_divine_damage = 40.0 + (power * 0.1)}

    let mut unmitigated_poly_damage = 0.0;
    if poly_bool{ unmitigated_poly_damage = (power * 0.75) + god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0))}

    let mut unmitigated_hydras_auto_damage = 0.0;
    if hydras_bool
    {         
        let unmitigated_damage_hydras = god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power;
        let unmitigated_damage_after_multis_hydras = unmitigated_damage_hydras * god.auto_progression[0] * 1.3;
        let crit_multi = 1.0 + clamp(build.crit_chance + boomerang_crit,0.0,1.0) * (0.75 + deathbringer_bonus - 0.35 * (god.name==HEIM.name) as i32 as f32) * ((0.0 * magical as i32 as f32) + (1.0 * !magical as i32 as f32));
        unmitigated_hydras_auto_damage = unmitigated_damage_after_multis_hydras * crit_multi;
    }

    let mut estaff_max_health_steal = 0.0;
    if estaff_bool{ estaff_max_health_steal = 0.06 * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health)}

    let mut conduit_true_damage = 0.0;
    if conduit_bool{ conduit_true_damage = 40.0;}

    let mut tablet_true_damage = 0.0;
    if tablet_bool{ tablet_true_damage = 0.05 * (build.mana + (god.base_mana + god.mana_per_level * (level - 1.0))); }

    let mut unmitigated_crusher_damage = 0.0;
    if crusher_bool
    { 
        unmitigated_crusher_damage = (power * 0.3) * number_of_abilities as f32;
        if number_of_abilities > 1
        {
            unmitigated_crusher_damage += (power * 0.15) * (number_of_abilities as f32 - 1.0);
        }
    }

    let mut unmitigated_arondight_damage = 0.0;
    if arondight_bool{ unmitigated_arondight_damage = 20.0 + (power * 0.5)}

    let mut unmitigated_bluestone_pendant_damage = 0.0;
    if bluestone_pendant_bool{ unmitigated_bluestone_pendant_damage = 40.0;}

    let mut unmitigated_bluestone_brooch_damage = 0.0;
    if bluestone_brooch_bool{ unmitigated_bluestone_brooch_damage = (160.0 + 0.12 * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health));}

    let mut unmitigated_redstone_damage = 0.0;
    if redstone_bool{ unmitigated_redstone_damage = 250.0;}

    let unmitigated_damage =(unmitigated_ability_damage_after_crit * (1.0+sacrificial_damage_multi)) + unmitigated_reaver_damage + unmitigated_soul_gem_damage + unmitigated_poly_damage + 
                                unmitigated_archmages_damage + unmitigated_crusher_damage + unmitigated_heartseeker_damage + unmitigated_arondight_damage + unmitigated_bluestone_pendant_damage + 
                                unmitigated_bluestone_brooch_damage + unmitigated_redstone_damage + unmitigated_hydras_auto_damage + unmitigated_calam_damage + unmitigated_claw_damage + unmitigated_divine_damage;

    let damage = (unmitigated_damage * protections_multiplier) + conduit_true_damage + mhb_damage + tablet_true_damage;

    let damage_after_you_do_more_damage_buffs = damage * (1.0  + focus_damage_multi + fighters_multi + rangdas_multi + myr_multi);

    let damage_after_increased_damage_on_target = damage_after_you_do_more_damage_buffs * (1.0 + magus_multi + runeforged_multi);

    let damage_after_mitigations = damage_after_increased_damage_on_target * (1.0 - mitigations - item_mitigations);

    damage_after_mitigations + estaff_max_health_steal
}

fn auto_attack_dps (time_to_auto:f32, god:&God, level: f32, build:&Build, target:&God, target_level:f32, target_build:&Build, 
    temper_enabled:bool,diamond_enabled:bool,focus_enabled:bool,magus_enabled:bool,doom_enabled:bool,binding_enabled:bool,
    bancroft_enabled:bool,tahuti_enabled:bool,magical:bool,serrated_enabled:bool,runeforged_enabled:bool,cowl_enabled:bool,
    redstone_attack_speed_enabled:bool,bumbas_spear_enabled:bool,protector_enabled:bool,kaldr_enabled:bool,pos2_enabled:bool,as_buff:f32,
    freya_1_2_enabled:bool,baka_3_enabled:bool,sol_passive:bool,obby_enabled:bool,flat_power_buff:f32,percent_power_buff:f32,red_buff:bool,
    purp_buff:bool,fire_giant:bool,enhanced_fire_giant:bool,p500_pot:bool,pen_pot:bool,mitigations:f32,chronos_2:bool,
    ao_2:bool,ferocious_enabled:bool,myr_enabled:bool,ishtar_1a:bool,ishtar_1b:bool,ishtar_1c:bool,boomerang_enabled:bool,
    vital_enabled:bool,spectral_aura_enabled:bool) -> f32
{
    let mut telk_bool = false;
    let mut demonic_bool = false;
    let mut basic_attack_damage = 0.0;
    let mut temper_multi = 0.0;
    let mut focus_damage_multi = 0.0;
    let mut magus_multi = 0.0;
    let mut binding_shred = 0.0;
    let mut typhon_bool = false;
    let mut bancroft_lifesteal = 0.0; //just used for any lifesteal gained through passives and not part of the build. Hasn't been renamed out of laziness
    let mut tahuti_scaling_multi = 1.0;

    let mut exe_bool = false;
    let mut heavy_exe_bool = false;
    let mut ferocious_exe_bool = false;
    let mut qins_bool = false;
    let mut deathbringer_bonus = 0.0;
    let mut obow_bool = false;
    let mut wind_bool = false;
    let mut dominance_pen = 0.0;
    let mut silverbranch_bool = false;
    let mut runeforged_multi = 0.0;
    let mut percent_power_increase = percent_power_buff;
    let mut manikins_scepter_bool = false;
    let mut manikins_mace_bool = false;
    let mut nimble_bool = false;
    let mut sacrificial_damage_multi = 0.0;
    let mut toxic_blade_bool = false;
    let mut obby_pen = 0.0;
    let mut myr_bool = false;
    let mut myr_multi = 0.0;
    let mut fighters_multi = 0.0;
    let mut rangdas_bool = false;
    let mut anim_bool = false;
    let mut cyclopean_bool = false;
    let mut time_of_next_cyclopean = 0.0;
    let mut toxic_blade_stacks = -1.0;
    let mut boomerang_crit = 0.0;
    let mut manti_poison_stacks = 0.0;
    let mut next_manti_poison_damage_time = 0.0;


    let mut power = clamp((build.magical_power+ ((god.class=="Mage")as i32 as f32) * (20.0 + level)) * magical as i32 as f32 + build.physical_power * !magical as i32 as f32 + flat_power_buff,0.0,900.0 * magical as i32 as f32 + 400.0 * !magical as i32 as f32);    
    let flat_pen = clamp(build.flat_pen + ((god.class=="Assassin")as i32 as f32) * (2.0 + 0.4 * level),0.0,50.0);
    let mut percent_pen = clamp(build.percent_pen,0.0,0.32);
    let crit_chance = clamp(build.crit_chance,0.0,1.0);
    let mut attack_speed = god.base_as + (god.as_per_level * (level - 1.0)) + build.attack_speed * (god.base_as - god.as_per_level);
    
    if god.name == KUKU.name {power += (build.mana + (god.base_mana + god.mana_per_level * (level - 1.0))) * 0.04; }
    if god.name == FREYA.name { bancroft_lifesteal += 0.07 + 0.002 * level; }

    if ishtar_1a
    {
        basic_attack_damage += 29.0;
        deathbringer_bonus -= 0.4;
    }

    let mut ishtar_1c_damage_reduction = 0.0;
    if ishtar_1c
    {
        attack_speed += 0.4 * (god.base_as - god.as_per_level);
        ishtar_1c_damage_reduction = 0.2;
    }

    let mut sol_passive_auto_multiplier = 1.0;
    if sol_passive 
    { 
        percent_power_increase += 0.25;
        attack_speed += 0.3 * (god.base_as - god.as_per_level);
        sol_passive_auto_multiplier = 1.15;
    }
    let mut chronos_2_auto_scaling_increase = 0.0;
    if chronos_2
    {
        attack_speed += 0.35 * (god.base_as - god.as_per_level);
        chronos_2_auto_scaling_increase = 0.35;
    }

    let mut ao_2_stacks = 0.0;
    if ao_2 { ao_2_stacks = 6.0; }

    for name in build.names.iter()
    {
        if name == &TYPHONS.name { typhon_bool = true;}
        if name == &TELKHINES.name { telk_bool = true;}
        if name == &DEMONIC_GRIP.name { demonic_bool = true;}
        if name == &GILDED.name { basic_attack_damage += 20.0; }
        if name == &DIAMOND.name { basic_attack_damage += 80.0; }
        if name == &ORNATE.name { basic_attack_damage += 70.0; }
        if name == &MANIKIN_SCEPTER.name { basic_attack_damage += 10.0; manikins_scepter_bool = true; }
        if name == &MANIKINS_MACE.name { basic_attack_damage += 50.0; manikins_mace_bool = true; }
        if name == &BOOK_OF_THOTH.name { power += build.mana * 0.1; }
        if temper_enabled && name == &DEATHS_TEMPER.name { temper_multi += 0.35; }
        if diamond_enabled && name == &DIAMOND.name { attack_speed += 0.75 * (god.base_as - god.as_per_level); }
        if focus_enabled && name == &FOCUS.name { focus_damage_multi = 0.09; }
        if magus_enabled && name == &MAGUS.name { magus_multi = 0.05; }
        if doom_enabled && name == &DOOM_ORB.name { power += 20.0; }
        if binding_enabled && name == &BINDING.name { binding_shred = 5.0 + level; }
        if bancroft_enabled && (name == &BANCROFTS.name || name == &NIMBLE_BANCROFTS.name || name == &BANCROFTS_CLAW.name) { power += 100.0; bancroft_lifesteal += 0.1; }
        if tahuti_enabled && (name == &TAHUTI.name || name == &CALAM_TAHUTI.name) { tahuti_scaling_multi = 1.2; }
        if name == &SACRIFICIAL.name { sacrificial_damage_multi = 0.15; }

        if name == &DOMINANCE.name { dominance_pen = 0.16; }
        if name == &EXE.name || name == &FEROCIOUS_EXE.name { exe_bool = true; }
        if name == &HEAVY_EXE.name { heavy_exe_bool = true; }
        if ferocious_enabled && name == &FEROCIOUS_EXE.name { ferocious_exe_bool = true}
        if name == &QINS.name { qins_bool = true; }
        if name == &DEATHBRINGER.name { deathbringer_bonus += 0.25}
        if name == &OBOW.name { obow_bool = true; }
        if name == &WIND_DEMON.name { wind_bool = true; }
        if name == &TRANS.name { power += (build.mana + (god.base_mana + god.mana_per_level * (level - 1.0))) * 0.03; }
        if name == &SILVERBRANCH.name { silverbranch_bool = true; }
        if serrated_enabled && name == &SERRATED.name { power += 30.0; }
        if runeforged_enabled && name == &RUNEFORGED.name { runeforged_multi = 0.1; }
        if cowl_enabled
        {
            if name== &LEATHER_COWL.name { attack_speed += 0.1 * (god.base_as - god.as_per_level);}
            if name== &HUNTERS_COWL.name { attack_speed += 0.2 * (god.base_as - god.as_per_level);}
        }
        if redstone_attack_speed_enabled && name == &REDSTONE.name 
        { 
            attack_speed += 0.5 * (god.base_as - god.as_per_level);
        }
        if bumbas_spear_enabled && name == &BUMBAS_SPEAR.name { percent_power_increase += 0.1; }
        if protector_enabled && name == &PROTECTOR.name {  percent_power_increase += 0.15}
        if name == &SPARTAN.name {  percent_power_increase += 0.1;}

        if name == &NIMBLE_BANCROFTS.name { nimble_bool = true; } 
        if name == &TOXICBLADE.name { toxic_blade_bool = true;}
        if name ==&OBBY.name && obby_enabled { obby_pen = 0.08;}

        if myr_enabled && name == &MYRDDIN.name { myr_bool = true; myr_multi = 0.2; }
        if name == &FIGHTERS.name { fighters_multi = 0.06 + (0.0025 * level); }
        if name == &RANGDAS.name { rangdas_bool = true; }
        if name == &ANIMOSITY.name { anim_bool = true; }
        if name == &CYCLOP_RING.name { cyclopean_bool = true; }
        if name == &TOXICBLADE.name { toxic_blade_stacks = 0.0;}
        if boomerang_enabled && name == &BOOMERANG.name { boomerang_crit = 0.3; }
        if vital_enabled && name == &VITAL_AMP.name { attack_speed += 0.3 * (god.base_as - god.as_per_level); temper_multi += 0.15; }
        
        
    }

    let mut spectral_multi = 0.0;
    let mut item_mitigations = 0.0;
    for name in target_build.names.iter()
    {
        if name == &SPECTRAL.name { spectral_multi = 0.3; }
        if name == &ONI_HUNTERS.name { item_mitigations += 0.09; }
    }
    if spectral_aura_enabled { spectral_multi = 0.3; }
    //if target.name == CABRAKAN.name { item_mitigations += 0.05; }

    attack_speed += as_buff * (god.base_as - god.as_per_level);

    if red_buff
    {
        percent_power_increase += 0.1;
        if magical{ power += 10.0; }
        else { power += 5.0; }
    }




    if purp_buff
    { 
        attack_speed += 0.1 * (god.base_as - god.as_per_level);
        if magical{ basic_attack_damage += 15.0; }
        else{ basic_attack_damage += 12.0; }
    }



    if fire_giant
    {
        if magical{ power += 70.0; }
        else { power += 50.0; }
    }

    if enhanced_fire_giant
    {
        if magical{ power += 100.0; }
        else { power += 65.0; }
    }

    if p500_pot
    {
        if magical{ power += 70.0; }
        else { power += 40.0; }
    }

    if pen_pot
    {
        percent_power_increase += 0.25;
        percent_pen = clamp(percent_pen + 0.08,0.0,0.32)
    }

    if typhon_bool { power += clamp(build.lifesteal + bancroft_lifesteal,0.0,0.4)*200.0}

    if nimble_bool { attack_speed += (1.0/2000.0) * clamp(power,0.0,800.0) * (god.base_as - god.as_per_level);}

    let mut time_spent_attacking = 0.0;
    let mut total_damage = 0.0;
    
    let mut silverbranch_power = 0.0;
    let mut demonic_stacks = 0.0;
    let mut exe_stacks = 0.0;
    let mut heavy_exe_stacks = 0.0;
    let mut ferocious_exe_stacks = 0.0;
    let mut obow_stacks = 0;
    let mut manikins_scepter_stacks = 0.0;
    let mut next_mani_scepter_damage_time = 0.0;
    let mut manikins_mace_stacks = 0.0;
    let mut next_mani_mace_damage_time = 0.0;
    let mut kaldr_alternator = 1.0; //as kaldrs attack speed is half skadis, he gets 1 attack every 2 skadi attacks
    let estimated_autos_to_proc_wind_demon = 1.0 / clamp(build.crit_chance + boomerang_crit,0.0,1.0);
    let mut autos = 0.0;
    let mut auto_attack_progression_counter = 0;
    let mut anhur_passive_shred = 0.0;

    let mut temp_percent_power_increase_store = 0.0;
    let mut temp_flat_power_increase_store = 0.0;

    power *= 1.0 + percent_power_increase;
    while time_spent_attacking < time_to_auto
    {
        power /= 1.0 + percent_power_increase;
        power -= silverbranch_power;
        if silverbranch_bool{ silverbranch_power = clamp(attack_speed - 2.5,0.0,99.9) * 100.0; } 
        percent_power_increase += temp_percent_power_increase_store;
        power += temp_flat_power_increase_store;
        temp_percent_power_increase_store = 0.0;
        temp_flat_power_increase_store = 0.0;
        power += silverbranch_power;
        power *= 1.0 + percent_power_increase;

        let mut rangdas_multi = 0.0;
        if rangdas_bool
        {
            if magical{ rangdas_multi = clamp(0.03 * ((power / 90.0) - (power % 90.0)/90.0), 0.0, 0.15); }
            else { rangdas_multi = clamp(0.03 * ((power / 65.0) - (power % 65.0)/65.0), 0.0, 0.15); }
        }

        let mut heim_progression_multi = 1.0; //heims first 2 autos have swing times and damage which don't match so this is used to account for it
        if god.name == HEIM.name && auto_attack_progression_counter < 2{ heim_progression_multi = 1.04; }
        if god.name == AOKUANG.name && auto_attack_progression_counter > 0 && auto_attack_progression_counter < 3{ heim_progression_multi = 1.4; }
        if god.name == CHARYBDIS.name && auto_attack_progression_counter > 1{ heim_progression_multi = 1.201; }

        let mut as_cap_reduction = 0.0;
        if heavy_exe_bool { as_cap_reduction = 0.6; }

        if myr_bool{ myr_multi = clamp((-0.028125*time_spent_attacking)+0.25,0.025,0.25); }
        time_spent_attacking += (1.0/clamp(attack_speed,0.35,2.5-as_cap_reduction)) * god.auto_progression[auto_attack_progression_counter] * heim_progression_multi;

        let autos_per_attack = 1.0 + 1.0 * (god.name == ZHONG.name) as i32 as f32; //used for zhong double autos
        let mut i = 0.0;
        while i < autos_per_attack
        {
            i += 1.0;
            let auto_attack_power_scaling = (0.2 * magical as i32 as f32) + (1.0 * !magical as i32 as f32) + (0.05 * (god.name==OLORUN.name) as i32 as f32) + chronos_2_auto_scaling_increase;         
            let unmitigated_damage = god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power * tahuti_scaling_multi * auto_attack_power_scaling;
            let unmitigated_damage_after_hunter_class_bonus = unmitigated_damage * (1.0 + (level * 0.005));
            let unmitigated_damage_after_multis = unmitigated_damage_after_hunter_class_bonus * god.auto_progression[auto_attack_progression_counter] * (1.0 + temper_multi);
            let crit_multi = clamp(1.0 + clamp(build.crit_chance + boomerang_crit,0.0,1.0) * (0.75 + deathbringer_bonus - spectral_multi - 0.35 * (god.name==HEIM.name) as i32 as f32 - 0.65 * (target.name==GEB.name) as i32 as f32) * !magical as i32 as f32,1.0,100.0);
            let olorun_crit_multi = 1.0 + clamp(power/1000.0,0.0,0.7) * 0.5 * (god.name==OLORUN.name) as i32 as f32 * (power>=150.0) as i32 as f32;
            let mut unmitigated_damage_after_crit = unmitigated_damage_after_multis * crit_multi * olorun_crit_multi * sol_passive_auto_multiplier * (1.0 - ishtar_1c_damage_reduction);
            if i == 2.0 { unmitigated_damage_after_crit *= 0.5; } //halves damage for zhongs mini auto

            
            let target_magical_prots = target.base_magical_prots + (target.magical_prots_per_level * (target_level - 1.0)) + target_build.magical_prot;
            let target_physical_prots = target.base_phys_prots + (target.phys_prots_per_level * (target_level - 1.0)) + target_build.phys_prot;
            let target_prots = target_magical_prots * magical as i32 as f32 + target_physical_prots * !magical as i32 as f32;
            let target_prots_after_shred = ((target_prots - binding_shred) * (1.0 - (0.09*demonic_stacks) - (0.06*exe_stacks) - (0.15*heavy_exe_stacks))) - anhur_passive_shred;
            let target_prots_after_pen = (target_prots_after_shred * (1.0 - percent_pen - dominance_pen)) - flat_pen;
            let protections_multiplier = clamp(100.0/(100.0+target_prots_after_pen),0.0,1.0);

            let mut unmitigated_telk_damage = 0.0;
            if telk_bool { unmitigated_telk_damage = 10.0 + (2.0*level);}

            let mut unmitigated_qins_damage = 0.0;  
            if qins_bool 
            {
                let percent_health = clamp((1.0/25000.0)*((target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health)) - (0.05),0.03,0.06);
                unmitigated_qins_damage = percent_health * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health);
                unmitigated_qins_damage *= 1.0 - 0.3 * (god.name == CHARYBDIS.name) as i32 as f32;
            }

            let mut unmitigated_cyclop_damage = 0.0;
            if cyclopean_bool
            {
                if time_of_next_cyclopean <= time_spent_attacking
                {
                    unmitigated_cyclop_damage = 0.07 * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health);
                    time_of_next_cyclopean = time_spent_attacking + 10.0
                }
                time_of_next_cyclopean -= 2.0;
            }

            let mut unmitigated_erlang_passive_damage = 0.0;
            if god.name == ERLANG.name
            {
                let erlang_passive_percent_damage = 0.0175 * (target.base_health + (target.health_per_level * (target_level - 1.0)) + target_build.health);
                let erlang_passive_flat_damage = (god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power * auto_attack_power_scaling) * 0.05;
                unmitigated_erlang_passive_damage = erlang_passive_flat_damage + erlang_passive_percent_damage;   
            }

            let mut unmitigated_obow_damage = 0.0;
            if obow_bool
            {            
                if obow_stacks == 3
                {
                    unmitigated_obow_damage = 15.0 + 0.6 * (god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power);
                    unmitigated_obow_damage *= 1.0 - 0.3 * (god.name == CHARYBDIS.name) as i32 as f32;
                }
                obow_stacks = (obow_stacks + 1) % 4;
            }

            let mut unmitigated_manti_poison_damage = 0.0;
            if god.name == MANTI.name
            {
                if(manti_poison_stacks == 0.0){ manti_poison_stacks = 4.0; next_manti_poison_damage_time = time_spent_attacking + 1.0; }
                else if(manti_poison_stacks < 8.0) { manti_poison_stacks+=1.0; }

                if time_spent_attacking > next_manti_poison_damage_time
                {
                    next_mani_scepter_damage_time = time_spent_attacking + 1.0;
                    unmitigated_manti_poison_damage = ((god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power)) * 0.025 * manti_poison_stacks;
                }
            }

            let mut manikins_scepter_damage = 0.0;
            let mut unmitigated_manikins_scepter_damage = 0.0;
            if manikins_scepter_bool
            {       
                if manikins_scepter_stacks == 0.0 { next_mani_scepter_damage_time = time_spent_attacking + 1.0; }     
                if time_spent_attacking > next_mani_scepter_damage_time
                {
                    next_mani_scepter_damage_time = time_spent_attacking + 1.0;
                    unmitigated_manikins_scepter_damage = (8.0 + 0.035 * power) * 0.5 * manikins_scepter_stacks;
                }
                if manikins_scepter_stacks < 3.0{ manikins_scepter_stacks += 1.0; }   
                
                if !magical
                {            
                    manikins_scepter_damage = unmitigated_manikins_scepter_damage * protections_multiplier;
                }
                else //for magical is a bit special since MHB does physical damage
                {
                    let target_physical_prots_after_shred_and_pen = (target_physical_prots - binding_shred) - (TOXICBLADE.flat_pen * toxic_blade_bool as i32 as f32);
                    let physical_protections_multiplier = clamp(100.0/(100.0+target_physical_prots_after_shred_and_pen),0.0,1.0);
                    manikins_scepter_damage = unmitigated_manikins_scepter_damage * physical_protections_multiplier;
                }
                manikins_scepter_damage *= 1.0 - 0.3 * (god.name == CHARYBDIS.name) as i32 as f32;
            }
            

            let mut unmitigated_manikins_mace_damage = 0.0;
            let mut manikins_mace_damage = 0.0;
            if manikins_mace_bool
            {       
                if manikins_mace_stacks == 0.0 { next_mani_mace_damage_time = time_spent_attacking + 1.0; }     
                if time_spent_attacking > next_mani_mace_damage_time
                {
                    next_mani_mace_damage_time = time_spent_attacking + 1.0;
                    unmitigated_manikins_mace_damage = (60.0) * 0.5 * manikins_mace_stacks;
                }
                if manikins_mace_stacks < 3.0{ manikins_mace_stacks += 1.0; }
                
                if !magical
                {            
                    manikins_mace_damage = unmitigated_manikins_mace_damage * protections_multiplier;
                }
                else //for magical is a bit special since MHB does physical damage
                {
                    let target_physical_prots_after_shred_and_pen = (target_physical_prots - binding_shred) - (TOXICBLADE.flat_pen * toxic_blade_bool as i32 as f32);
                    let physical_protections_multiplier = clamp(100.0/(100.0+target_physical_prots_after_shred_and_pen),0.0,1.0);
                    manikins_mace_damage = unmitigated_manikins_mace_damage * physical_protections_multiplier;
                }
                manikins_mace_damage *= 1.0 - 0.3 * (god.name == CHARYBDIS.name) as i32 as f32;
            }

            let mut anim_damage = 0.0;
            if anim_bool
            {       
                let unmitigated_anim_damage = 0.025 * (god.base_health + (god.health_per_level * (level - 1.0)) + build.health);                 
                
                if magical
                {            
                    anim_damage = unmitigated_anim_damage * protections_multiplier;
                }
                else //for magical is a bit special since anim does magical damage
                {
                    let target_magical_prots_after_shred_and_pen = (target_physical_prots - binding_shred) - (TOXICBLADE.flat_pen * toxic_blade_bool as i32 as f32);
                    let magical_protections_multiplier = clamp(100.0/(100.0+target_magical_prots_after_shred_and_pen),0.0,1.0);
                    anim_damage = unmitigated_anim_damage * magical_protections_multiplier;
                }
                manikins_scepter_damage *= 1.0 - 0.3 * (god.name == CHARYBDIS.name) as i32 as f32;
            }

            let mut unmitigated_damage_kaldr_damage = 0.0;
            kaldr_alternator *= -1.0;
            if kaldr_enabled && kaldr_alternator == 1.0
            {
                unmitigated_damage_kaldr_damage = (god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power * auto_attack_power_scaling) * 0.8;
            }

            let mut unmitigated_freya_1_damage = 0.0;
            let mut mitigated_freya_2_damage = 0.0;
            if freya_1_2_enabled
            { 
                unmitigated_freya_1_damage = (100.0 + (power * 0.25 * tahuti_scaling_multi)) * (1.0 + sacrificial_damage_multi);
                let unmitigated_freya_2_damage = (70.0 + (power * 0.15 * tahuti_scaling_multi)) * (1.0 + sacrificial_damage_multi);
                
                let target_prots_after_pen_with_obby = (target_prots_after_shred * (1.0 - percent_pen - dominance_pen - obby_pen)) - flat_pen;
                let protections_multiplier_with_obby = clamp(100.0/(100.0+target_prots_after_pen_with_obby),0.0,1.0);
                mitigated_freya_2_damage = unmitigated_freya_2_damage * protections_multiplier_with_obby
                
            }

            let mut mitigated_ao_2_damage = 0.0;
            if ao_2_stacks > 0.0
            {
                ao_2_stacks -= 1.0;
                let unmitigated_ao_2_damage = (95.0 + (power * 0.3 * tahuti_scaling_multi)) * (1.0 + sacrificial_damage_multi);
                let target_prots_after_pen_with_obby = (target_prots_after_shred * (1.0 - percent_pen - dominance_pen - obby_pen)) - flat_pen;
                let protections_multiplier_with_obby = clamp(100.0/(100.0+target_prots_after_pen_with_obby),0.0,1.0);
                mitigated_ao_2_damage = unmitigated_ao_2_damage * protections_multiplier_with_obby;
            }

            let mut unmitigated_pos2_damage = 0.0;
            if pos2_enabled{ unmitigated_pos2_damage = 2.0 * (70.0 + (power * 0.25));} // *2 as we assume both 'extra' shots hit

            let mut unmitigated_ishtar_1b_damage = 0.0;
            if ishtar_1b
            { 
                let crit_multi = clamp(1.0 + clamp(build.crit_chance + boomerang_crit,0.0,1.0) * (0.75 + deathbringer_bonus - spectral_multi - 0.35 * (god.name==HEIM.name) as i32 as f32 - 0.65 * (target.name==GEB.name) as i32 as f32) * !magical as i32 as f32,1.0,100.0);
                unmitigated_ishtar_1b_damage = 0.32 * (god.base_auto_damage + (god.auto_damage_per_level * (level - 1.0)) + basic_attack_damage + power) * crit_multi; 
            }

            let mut butcher_blades_damage = 0.0;
            if baka_3_enabled { butcher_blades_damage = 70.0; }

            if god.name == ANHUR.name && anhur_passive_shred == 0.0 { anhur_passive_shred = 20.0; }



            let damage = (unmitigated_damage_after_crit + unmitigated_telk_damage + unmitigated_qins_damage + unmitigated_obow_damage 
                        + unmitigated_damage_kaldr_damage +  unmitigated_pos2_damage + unmitigated_freya_1_damage + unmitigated_erlang_passive_damage 
                        + unmitigated_ishtar_1b_damage + unmitigated_cyclop_damage + unmitigated_manti_poison_damage) * protections_multiplier 
                        + manikins_mace_damage + manikins_scepter_damage + butcher_blades_damage + mitigated_freya_2_damage + mitigated_ao_2_damage + anim_damage;
            

            let damage_after_you_do_more_damage_buffs = damage * (1.0 + focus_damage_multi + myr_multi + fighters_multi + rangdas_multi);

            let damage_after_increased_damage_on_target = damage_after_you_do_more_damage_buffs * (1.0 + magus_multi + runeforged_multi + (ferocious_exe_stacks * 0.02));

            let damage_after_mitigations = damage_after_increased_damage_on_target * (1.0 - mitigations - item_mitigations);

            total_damage += damage_after_mitigations;

            




            if demonic_bool && demonic_stacks < 3.0 { demonic_stacks = demonic_stacks + 1.0; }

            if exe_bool && exe_stacks < 4.0 { exe_stacks += 1.0; }

            if heavy_exe_bool && heavy_exe_stacks < 2.0 { heavy_exe_stacks += 1.0; }

            if ferocious_exe_bool && ferocious_exe_stacks < 10.0 { ferocious_exe_stacks += 1.0; }
            
            if (toxic_blade_stacks != -1.0) && (toxic_blade_stacks < 4.0)
            {
                attack_speed += 0.05 * (god.base_as - god.as_per_level);
                toxic_blade_stacks += 1.0;
            }

            


            autos += 1.0;
            if wind_bool && autos >= estimated_autos_to_proc_wind_demon
            {
                percent_pen = clamp(percent_pen + 0.08,0.0,0.32);
                attack_speed += 0.1 * (god.base_as - god.as_per_level); 
                wind_bool = false;
            }

            auto_attack_progression_counter = (auto_attack_progression_counter + 1) % 7;
            if god.auto_progression[auto_attack_progression_counter] == 0.0 { auto_attack_progression_counter = 0}
        }
        
    }

    total_damage / time_spent_attacking
}


//-------------------------------------------------------------------------
//Main


fn main() {
    let app = app::App::default();

    let mut wind = Window::new(0, 0, 1200, 470, "Smite Damage Stuff 10.5");


    let mut run_btn = Button::new(10, 0, 80, 40, "Run");
    let mut base_damage_input = input::IntInput::new(200,0,40,40,"Base Damage: ");
    base_damage_input.set_value("400");
    let mut scaling_input = input::FloatInput::new(300,0,40,40,"Scaling: ");
    scaling_input.set_value("0.7");

    let mut auto_sample_time_input = input::FloatInput::new(470,0,40,40,"Auto Sample Time: ");
    auto_sample_time_input.set_value("2.0");

    let mut sort_damage_menu = menu::Choice::new(620,0,200,40,"Sort Builds By:");
    sort_damage_menu.add_choice("No Sort|Ability Damage (Target 1)|Auto DPS (Target 1)|Ability Damage (Target 2)|Auto DPS (Target 2)");
    sort_damage_menu.set_value(0);

    let mut number_of_abilities_input = input::IntInput::new(950,0,40,40,"Number of Abilities: ");
    number_of_abilities_input.set_value("1");

    let mut builds_to_store_with_sort = input::IntInput::new(710,40,40,40,"Builds Displayed When Sorting: ");
    builds_to_store_with_sort.set_value("20");

    let mut ability_damage_enable_btn = button::CheckButton::new(10,50,120,30,"Ability Damage");
    ability_damage_enable_btn.set_value(true);
    let mut auto_dps_enable_btn = button::CheckButton::new(150,50,90,30,"Auto DPS");
    auto_dps_enable_btn.set_value(true);
    let ttk_btn = button::CheckButton::new(250,50,230,30,"Display TTK or % Hp Damage");

    let mut pack = group::Pack::new(10, 100, 100, 30, "Toggle Item Passives:");
    pack.set_type(group::PackType::Horizontal);
    pack.set_spacing(10);
    let obby_btn = button::CheckButton::default().with_size(120,0).with_label("Obsidian Shard");
    let soul_gem_btn = button::CheckButton::default().with_size(80,0).with_label("Soul Gem"); 
    let poly_btn = button::CheckButton::default().with_size(100,0).with_label("Polynomicon");
    let estaff_btn = button::CheckButton::default().with_size(110,0).with_label("Ethereal Staff"); 
    let conduit_btn = button::CheckButton::default().with_size(100,0).with_label("Conduit Gem"); 
    let archmage_btn = button::CheckButton::default().with_size(130,0).with_label("Archmage's Gem"); 
    let bancrofts_claw_btn = button::CheckButton::default().with_size(130,0).with_label("Bancroft's Claw");
    let book_of_thoth_btn = button::CheckButton::default().with_size(110,0).with_label("Book of Thoth"); 
    let warlocks_btn = button::CheckButton::default().with_size(120,0).with_label("Warlock's Staff");     
    pack.end();

    let mut phys_pack = group::Pack::new(10, 100, 100, 30, "Toggle Item Passives:");
    phys_pack.set_type(group::PackType::Horizontal);
    phys_pack.set_spacing(10);
    let titans_bane_btn = button::CheckButton::default().with_size(100,0).with_label("Titan's Bane");
    let arondight_btn = button::CheckButton::default().with_size(90,0).with_label("Arondight");
    let serrated_btn = button::CheckButton::default().with_size(110,0).with_label("Serrated Edge");
    let runeforged_btn = button::CheckButton::default().with_size(150,0).with_label("Runeforged Hammer");
    let cowl_btn = button::CheckButton::default().with_size(260,0).with_label("Leather/Hunter's Cowl (Attack Speed)");
    let ferocious_btn = button::CheckButton::default().with_size(170,0).with_label("Ferocious Executioner");
    phys_pack.end();
    phys_pack.hide();

    let mut pack2 = group::Pack::new(10, 130, 100, 30, "");
    pack2.set_type(group::PackType::Horizontal);
    pack2.set_spacing(10);
    let temper_btn = button::CheckButton::default().with_size(120,0).with_label("Death's Temper");
    let diamond_btn = button::CheckButton::default().with_size(120,0).with_label("Diamond Arrow");
    let bumbas_spear_btn = button::CheckButton::default().with_size(120,0).with_label("Bumba's Spear");
    let protector_btn = button::CheckButton::default().with_size(170,0).with_label("Protector of the Jungle");
    let mhb_btn = button::CheckButton::default().with_size(160,0).with_label("Manikin Hidden Blade");
    let binding_btn = button::CheckButton::default().with_size(130,0).with_label("Stone of Binding");
    pack2.end();   


    let mut pack3 = group::Pack::new(10, 160, 100, 30, "");
    pack3.set_type(group::PackType::Horizontal);
    pack3.set_spacing(10);
    let focus_btn = button::CheckButton::default().with_size(110,0).with_label("Gem of Focus");
    let magus_btn = button::CheckButton::default().with_size(150,0).with_label("Spear of the Magus");
    let doom_btn = button::CheckButton::default().with_size(90,0).with_label("Doom Orb");
    let bancroft_btn = button::CheckButton::default().with_size(130,0).with_label("Bancroft's Talon");
    let tahuti_btn = button::CheckButton::default().with_size(110,0).with_label("Rod of Tahuti");
    let calam_btn = button::CheckButton::default().with_size(130,0).with_label("Calamitous Glyph");
    let myr_btn = button::CheckButton::default().with_size(70,0).with_label("Myrddin"); 
    let divine_btn = button::CheckButton::default().with_size(60,0).with_label("Divine"); 
    let tablet_btn = button::CheckButton::default().with_size(140,0).with_label("Tablet of Destinies"); 
    pack3.end();

    let mut phys_pack3 = group::Pack::new(10, 160, 100, 30, "");
    phys_pack3.set_type(group::PackType::Horizontal);
    phys_pack3.set_spacing(10);
    let redstone_attack_speed_btn = button::CheckButton::default().with_size(250,0).with_label("Corrupted Bluestone (Attack Speed)");
    let trans_btn = button::CheckButton::default().with_size(120,0).with_label("Transcendence");
    let devos_btn = button::CheckButton::default().with_size(100,0).with_label("Devourer's");
    let hydras_btn = button::CheckButton::default().with_size(120,0).with_label("Hydra's Lament");
    let boomerang_btn = button::CheckButton::default().with_size(140,0).with_label("Bladed Boomerang");
    let vital_btn = button::CheckButton::default().with_size(110,0).with_label("Vital Amplifier");

    phys_pack3.end();
    phys_pack3.hide();

    let mut level = valuator::HorNiceSlider::new(10,200,200,10,"Level: 20");
    level.set_minimum(1.);
    level.set_maximum(20.);
    level.set_step(1., 1); 
    level.set_value(20.);

    let mut target1_level = valuator::HorNiceSlider::new(220,200,200,10,"Target 1 Level: 20");
    target1_level.set_minimum(1.);
    target1_level.set_maximum(20.);
    target1_level.set_step(1., 1); 
    target1_level.set_value(20.);

    let mut target2_level = valuator::HorNiceSlider::new(430,200,200,10,"Target 2 Level: 20");
    target2_level.set_minimum(1.);
    target2_level.set_maximum(20.);
    target2_level.set_step(1., 1); 
    target2_level.set_value(20.);   

    let mut magical_item_pack = group::Pack::new(60, 230, 100, 30, "");
    magical_item_pack.set_type(group::PackType::Horizontal);
    magical_item_pack.set_spacing(10);

    let mut starter = menu::Choice::new(60,230,100,30,"Starter:");
    starter.add_choice(&array_to_menu_string(&MAGICAL_STARTERS).to_string());

    frame::Frame::default().with_size(50, 0); // a filler

    let mut item1 = menu::Choice::new(230,230,100,30,"Items:");
    item1.add_choice(&array_to_menu_string(&MAGICAL_ITEMS).to_string());

    let mut item2 = menu::Choice::new(340,230,100,30,"");
    item2.add_choice(&array_to_menu_string(&MAGICAL_ITEMS).to_string());

    let mut item3 = menu::Choice::new(450,230,100,30,"");
    item3.add_choice(&array_to_menu_string(&MAGICAL_ITEMS).to_string());

    let mut item4 = menu::Choice::new(560,230,100,30,"");
    item4.add_choice(&array_to_menu_string(&MAGICAL_ITEMS).to_string());

    let mut item5 = menu::Choice::new(670,230,100,30,"");
    item5.add_choice(&array_to_menu_string(&MAGICAL_ITEMS).to_string());
    magical_item_pack.end();

    let mut phys_item_pack = group::Pack::new(60, 230, 100, 30, "");
    phys_item_pack.set_type(group::PackType::Horizontal);
    phys_item_pack.set_spacing(10);

    let mut phys_starter = menu::Choice::new(60,230,100,30,"Starter:");
    phys_starter.add_choice(&array_to_menu_string(&PHYSICAL_STARTERS).to_string());

    frame::Frame::default().with_size(50, 0); // a filler

    let mut phys_item1 = menu::Choice::new(230,230,100,30,"Items:");
    phys_item1.add_choice(&array_to_menu_string(&PHYSICAL_ITEMS).to_string());

    let mut phys_item2 = menu::Choice::new(340,230,100,30,"");
    phys_item2.add_choice(&array_to_menu_string(&PHYSICAL_ITEMS).to_string());

    let mut phys_item3 = menu::Choice::new(450,230,100,30,"");
    phys_item3.add_choice(&array_to_menu_string(&PHYSICAL_ITEMS).to_string());

    let mut phys_item4 = menu::Choice::new(560,230,100,30,"");
    phys_item4.add_choice(&array_to_menu_string(&PHYSICAL_ITEMS).to_string());

    let mut phys_item5 = menu::Choice::new(670,230,100,30,"");
    phys_item5.add_choice(&array_to_menu_string(&PHYSICAL_ITEMS).to_string());
    phys_item_pack.end();
    phys_item_pack.hide();

    let mut physical_magical_toggle_btn = Button::new(800,220,160,30, "Physical/Magical Toggle");


    let mut target1_item1 = menu::Choice::new(230,260,100,30,"Target 1 Items:");
    target1_item1.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target1_item2 = menu::Choice::new(340,260,100,30,"");
    target1_item2.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target1_item3 = menu::Choice::new(450,260,100,30,"");
    target1_item3.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target1_item4 = menu::Choice::new(560,260,100,30,"");
    target1_item4.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target1_item5 = menu::Choice::new(670,260,100,30,"");
    target1_item5.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target1_item6 = menu::Choice::new(780,260,100,30,"");
    target1_item6.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());



    let mut target2_item1 = menu::Choice::new(230,290,100,30,"Target 2 Items:");
    target2_item1.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target2_item2 = menu::Choice::new(340,290,100,30,"");
    target2_item2.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());    

    let mut target2_item3 = menu::Choice::new(450,290,100,30,"");
    target2_item3.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target2_item4 = menu::Choice::new(560,290,100,30,"");
    target2_item4.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target2_item5 = menu::Choice::new(670,290,100,30,"");
    target2_item5.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    let mut target2_item6 = menu::Choice::new(780,290,100,30,"");
    target2_item6.add_choice(&array_to_menu_string(&TANK_ITEMS).to_string());

    //default build displayed
    target2_item1.set_value(39);
    target2_item2.set_value(18);
    target2_item3.set_value(15);
    target2_item4.set_value(8);
    target2_item5.set_value(5);
    target2_item6.set_value(0);

    let mut target1_min_ability_damage = input::IntInput::new(230,330,50,40,"Minimum Ability Damage: Target 1: ");
    target1_min_ability_damage.set_value("0");

    let mut target2_min_ability_damage = input::IntInput::new(350,330,50,40,"Target 2: ");
    target2_min_ability_damage.set_value("0");

    let mut target1_min_auto_dps = input::IntInput::new(600,330,50,40,"Minimum Auto DPS: Target 1: ");
    target1_min_auto_dps.set_value("0");

    let mut target2_min_auto_dps = input::IntInput::new(720,330,50,40,"Target 2: ");
    target2_min_auto_dps.set_value("0");

    let mut min_cdr = input::FloatInput::new(110,380,50,40,"Minimum CDR: ");
    min_cdr.set_value("0.0");

    let mut min_lifesteal = input::FloatInput::new(290,380,50,40,"Minimum Lifesteal: ");
    min_lifesteal.set_value("0.0");

    let require_antiheal = button::CheckButton::new(350,380,80,40,"Antiheal");

    let mut max_gold = input::FloatInput::new(540,380,80,40,"Maximum Gold: ");
    max_gold.set_value("30000.0");

    let mut god = input::Input::new(50,420,100,40,"God: ");
    god.set_value("Agni");

    let mut target1_god = input::Input::new(260,420,100,40,"Target 1 God: ");
    target1_god.set_value("Agni");

    let mut target2_god = input::Input::new(460,420,100,40,"Target 2 God: ");
    target2_god.set_value("Ares");

    let mut god_specific_btn = Button::new(600,420,160,40, "God Specific Options");

    let mut stat_buffs_btn = Button::new(770,420,80,40, "Stat Buffs");

    

    wind.end();
    wind.show();

    let mut wind2 = Window::new(500, 0, 500, 300, "God Specifics");
    let kaldr_btn = button::CheckButton::new(0, 0, 70, 30, "Kaldr");
    let pos2_btn = button::CheckButton::new(70, 0, 100, 30, "Poseidon 2");
    let ability_can_crit_btn = button::CheckButton::new(170, 0, 90, 30, "Ability Crit");
    let true_damage_ability = button::CheckButton::new(260, 0, 150, 30, "Ability True Damage");
    let freya_1_2 = button::CheckButton::new(0, 30, 100, 30, "Freya 1 + 2");
    let baka_3 = button::CheckButton::new(100, 30, 75, 30, "Baka 3");
    let sol_passive = button::CheckButton::new(175, 30, 110, 30, "Sol Passive");
    let chronos_2 = button::CheckButton::new(285, 30, 200, 30, "Chronos 2 (4th Quadrant)");
    let ao_2 = button::CheckButton::new(0, 60, 100, 30, "Ao Kuang 2");
    let ishtar_1a = button::CheckButton::new(100, 60, 90, 30, "Ishtar 1A");
    let ishtar_1b = button::CheckButton::new(190, 60, 90, 30, "Ishtar 1B");
    let ishtar_1c = button::CheckButton::new(280, 60, 90, 30, "Ishtar 1C");
    wind2.end();

    let mut wind3 = Window::new(500, 0, 500, 300, "Stat Buffs");
    let mut as_buff = input::FloatInput::new(140,0,50,40,"Attack Speed Buff: ");
    as_buff.set_value("0.0");
    let mut flat_power_buff = input::FloatInput::new(140,40,50,40,"Flat Power Buff: ");
    flat_power_buff.set_value("0.0");
    let mut percent_power_buff = input::FloatInput::new(140,80,50,40,"Percent Power Buff: ");
    percent_power_buff.set_value("0.0");
    let mut target1_mitigations = input::FloatInput::new(140,120,50,40,"Target 1 Mitigations: ");
    target1_mitigations.set_value("0.0");
    let mut target2_mitigations = input::FloatInput::new(140,160,50,40,"Target 2 Mitigations: ");
    target2_mitigations.set_value("0.0");


    let red_buff = button::CheckButton::new(220,0,80,40,"Red Buff");
    let purp_buff = button::CheckButton::new(220,40,100,40,"Purple Buff");
    let fire_giant = button::CheckButton::new(220,80,100,40,"Fire Giant");
    let enhanced_fire_giant = button::CheckButton::new(320,80,160,40,"Enhanced Fire Giant");
    let p500_pot = button::CheckButton::new(220,120,100,40,"500 Pot");
    let pen_pot = button::CheckButton::new(320,120,160,40,"Pen Pot");
    let spectral_aura = button::CheckButton::new(220,160,160,40,"Spectral Aura On Target");

    wind3.end();
    
    

    level.set_callback(|s| {
        s.set_label(&("Level: ".to_string() + &s.value().to_string()));        
    });

    target1_level.set_callback(|s| {
        s.set_label(&("Target 1 Level: ".to_string() + &s.value().to_string()));
    });

    target2_level.set_callback(|s| {
        s.set_label(&("Target 2 Level: ".to_string() + &s.value().to_string()));
    });

    god_specific_btn.set_callback(move |_| wind2.show());
    stat_buffs_btn.set_callback(move |_| wind3.show());

    physical_magical_toggle_btn.set_callback(move |physical_magical_toggle_btn| 
        { 
            physical_magical_toggle_btn.set_value(!physical_magical_toggle_btn.value());

            if physical_magical_toggle_btn.value()
            {
                pack.hide();
                pack3.hide(); 
                magical_item_pack.hide();
                
                phys_item_pack.show();
                phys_pack.show();
                phys_pack3.show();

            }
            else 
            {
                pack.show();
                pack3.show();
                magical_item_pack.show();

                phys_item_pack.hide();
                phys_pack.hide();
                phys_pack3.hide();
            }
        }
    );

    run_btn.set_callback(move |_| run(ability_damage_enable_btn.value(),auto_dps_enable_btn.value(),
            auto_sample_time_input.value().parse().unwrap(),base_damage_input.value().parse().unwrap() , 
            scaling_input.value().parse().unwrap() ,
            (&starter).value(),(&item1).value(),(&item2).value(),(&item3).value(),(&item4).value(),(&item5).value(),
            (&phys_starter).value(),(&phys_item1).value(),(&phys_item2).value(),(&phys_item3).value(),(&phys_item4).value(),(&phys_item5).value(),
            level.value() as f32,target1_level.value() as f32, target2_level.value() as f32,
            obby_btn.value(),soul_gem_btn.value(),poly_btn.value(),estaff_btn.value(),conduit_btn.value(),archmage_btn.value(),
            temper_btn.value(),diamond_btn.value(),focus_btn.value(),magus_btn.value(),doom_btn.value(),
            binding_btn.value(),bancroft_btn.value(),tahuti_btn.value(),(&target1_item1).value(),(&target1_item2).value(),(&target1_item3).value(),(&target1_item4).value(),
            (&target1_item5).value(),(&target1_item6).value(),(&target2_item1).value(),(&target2_item2).value(),(&target2_item3).value(),(&target2_item4).value(),
            (&target2_item5).value(),(&target2_item6).value(),target1_min_ability_damage.value().parse().unwrap(),target1_min_auto_dps.value().parse().unwrap()
            ,target2_min_ability_damage.value().parse().unwrap(),target2_min_auto_dps.value().parse().unwrap(),sort_damage_menu.value() as usize,builds_to_store_with_sort.value().parse().unwrap()
            ,!physical_magical_toggle_btn.value(),titans_bane_btn.value(),arondight_btn.value(),serrated_btn.value(),runeforged_btn.value(),cowl_btn.value(),
            redstone_attack_speed_btn.value(),bumbas_spear_btn.value(),protector_btn.value(),mhb_btn.value(),book_of_thoth_btn.value(),warlocks_btn.value(),trans_btn.value(),
            min_cdr.value().parse().unwrap(),min_lifesteal.value().parse().unwrap(),require_antiheal.value(),string_to_god(god.value().as_str()),
            string_to_god(target1_god.value().as_str()),string_to_god(target2_god.value().as_str()),kaldr_btn.value(),pos2_btn.value()
            ,ability_can_crit_btn.value(),true_damage_ability.value(),hydras_btn.value(),calam_btn.value(),as_buff.value().parse().unwrap(),freya_1_2.value(),
            baka_3.value(),sol_passive.value(),flat_power_buff.value().parse().unwrap(),
            percent_power_buff.value().parse().unwrap(),red_buff.value(),purp_buff.value(),fire_giant.value(),
            enhanced_fire_giant.value(),max_gold.value().parse().unwrap(),p500_pot.value(),pen_pot.value(),ttk_btn.value(),target1_mitigations.value().parse().unwrap()
            ,target2_mitigations.value().parse().unwrap(),chronos_2.value(),ao_2.value(),ferocious_btn.value(),bancrofts_claw_btn.value(),myr_btn.value(),
            ishtar_1a.value(),ishtar_1b.value(),ishtar_1c.value(),number_of_abilities_input.value().parse().unwrap(),divine_btn.value(),tablet_btn.value(),boomerang_btn.value(),
            devos_btn.value(),vital_btn.value(),spectral_aura.value())); 

    


    app.run().unwrap();
}




fn run(calculate_ability_damage:bool,calculate_auto_dps:bool,auto_sample_time:f32,base_ability_damage:f32,ability_scaling:f32,
    starter_index:i32,item1_index:i32,item2_index:i32,item3_index:i32,item4_index:i32,item5_index:i32,
    phys_starter_index:i32,phys_item1_index:i32,phys_item2_index:i32,phys_item3_index:i32,phys_item4_index:i32,phys_item5_index:i32,
    level:f32,target1_level:f32,target2_level:f32,obby_enabled:bool,soul_gem_enabled:bool,poly_enabled:bool,estaff_enabled:bool,
    conduit_enabled:bool,archmages_enabled:bool,temper_enabled:bool,diamond_enabled:bool,focus_enabled:bool,magus_enabled:bool,doom_enabled:bool,binding_enabled:bool,
    bancroft_enabled:bool,tahuti_enabled:bool,target1_item1_index:i32,target1_item2_index:i32,target1_item3_index:i32,target1_item4_index:i32,target1_item5_index:i32,target1_item6_index:i32
    ,target2_item1_index:i32,target2_item2_index:i32,target2_item3_index:i32,target2_item4_index:i32,target2_item5_index:i32,target2_item6_index:i32,
    target1_min_ability_damage:f32,target1_min_auto_dps:f32,target2_min_ability_damage:f32,target2_min_auto_dps:f32,sort_mode:usize,builds_to_store_with_sort:usize,magical:bool,
    titans_bane_enabled:bool,arondight_enabled:bool,serrated_enabled:bool,runeforged_enabled:bool,cowl_enabled:bool,redstone_attack_speed_enabled:bool,
    bumbas_spear_enabled:bool,protector_enabled:bool,mhb_enabled:bool,book_of_thoth_enabled:bool,warlocks_enabled:bool,trans_enabled:bool,min_cdr:f32,
    min_lifesteal:f32,require_antiheal:bool,god:&God,target1_god:&God,target2_god:&God,kaldr_enabled:bool,pos2_enabled:bool,
    ability_crit_enabled:bool,ability_true_damage:bool,hydras_enabled:bool,calam_enabled:bool,as_buff:f32,freya_1_2_enabled:bool,baka_3_enabled:bool,
    sol_passive:bool,flat_power_buff:f32,percent_power_buff:f32,red_buff:bool,purp_buff:bool,
    fire_giant:bool,enhanced_fire_giant:bool,max_gold:f32,p500_pot:bool,pen_pot:bool,ttk_display:bool,target1_mitigations:f32,target2_mitigations:f32,chronos_2:bool,
    ao_2:bool,ferocious_enabled:bool,claw_enabled:bool,myr_enabled:bool,ishtar_1a:bool,ishtar_1b:bool,ishtar_1c:bool,number_of_abilities:i32,divine_enabled:bool,tablet_enabled:bool,
    boomerang_enabled:bool,devos_enabled:bool,vital_enabled:bool,spectral_aura_enabled:bool)
{

    if god.name == ERROR.name || target1_god.name == ERROR.name || target2_god.name == ERROR.name
    {
        println!("God name spelt incorrectly");    
    }

    else
    {
        let mut required_items = Vec::new();
        if magical
        {
            if starter_index > 0
            {
                required_items.push(MAGICAL_STARTERS[(starter_index - 1) as usize])
            }
            if item1_index > 0
            {
                required_items.push(MAGICAL_ITEMS[(item1_index - 1) as usize])
            }
            if item2_index > 0
            {
                required_items.push(MAGICAL_ITEMS[(item2_index - 1) as usize])
            }
            if item3_index > 0
            {
                required_items.push(MAGICAL_ITEMS[(item3_index - 1) as usize])
            }
            if item4_index > 0
            {
                required_items.push(MAGICAL_ITEMS[(item4_index - 1) as usize])
            }
            if item5_index > 0
            {
                required_items.push(MAGICAL_ITEMS[(item5_index - 1) as usize])
            }
        }

        else 
        {
            if phys_starter_index > 0
            {
                required_items.push(PHYSICAL_STARTERS[(phys_starter_index - 1) as usize])
            }
            if phys_item1_index > 0
            {
                required_items.push(PHYSICAL_ITEMS[(phys_item1_index - 1) as usize])
            }
            if phys_item2_index > 0
            {
                required_items.push(PHYSICAL_ITEMS[(phys_item2_index - 1) as usize])
            }
            if phys_item3_index > 0
            {
                required_items.push(PHYSICAL_ITEMS[(phys_item3_index - 1) as usize])
            }
            if phys_item4_index > 0
            {
                required_items.push(PHYSICAL_ITEMS[(phys_item4_index - 1) as usize])
            }
            if phys_item5_index > 0
            {
                required_items.push(PHYSICAL_ITEMS[(phys_item5_index - 1) as usize])
            }
        }


        let mut target1_build_vec = Vec::new();
        if target1_item1_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item1_index - 1) as usize])
        }
        if target1_item2_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item2_index - 1) as usize])
        }
        if target1_item3_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item3_index - 1) as usize])
        }
        if target1_item4_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item4_index - 1) as usize])
        }
        if target1_item5_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item5_index - 1) as usize])
        }
        if target1_item6_index > 0
        {
            target1_build_vec.push(TANK_ITEMS[(target1_item6_index - 1) as usize])
        }
        for i in 0..(6-target1_build_vec.len()) { target1_build_vec.push(&EMPTY)}
        let target1_build_array:[&Item;6] = target1_build_vec[..].try_into().unwrap();
        let target1_build = combine_items(target1_build_array);


        let mut target2_build_vec = Vec::new();
        if target2_item1_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item1_index - 1) as usize])
        }
        if target2_item2_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item2_index - 1) as usize])
        }
        if target2_item3_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item3_index - 1) as usize])
        }
        if target2_item4_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item4_index - 1) as usize])
        }
        if target2_item5_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item5_index - 1) as usize])
        }
        if target2_item6_index > 0
        {
            target2_build_vec.push(TANK_ITEMS[(target2_item6_index - 1) as usize])
        }
        for i in 0..(6-target2_build_vec.len()) { target2_build_vec.push(&EMPTY)}
        let target2_build_array:[&Item;6] = target2_build_vec[..].try_into().unwrap();
        let target2_build = combine_items(target2_build_array);

        
        let mut sorted_builds = Vec::new();
        sorted_builds.push(["Sorter Min".to_string(),"0".to_string(),"0".to_string(),"0".to_string(),"0".to_string()]);
        sorted_builds.push(["Sorter Max".to_string(),"99999".to_string(),"99999".to_string(),"99999".to_string(),"99999".to_string()]);

        
        let mut iterator = MAGICAL_ITEMS.iter().combinations(6-required_items.len());
        if !magical { iterator = PHYSICAL_ITEMS.iter().combinations(6-required_items.len());}
        for permutation in iterator
        {
            let mut build_vec = required_items.clone();
            let mut duplicate_detected_in_build = false; //also used for whether a disabled item is detected (such as book of thoth)
            let mut bancroft_passive_lifesteal = 0.0; //used to see if a build satisfies the minimum lifesteal with bancroft enabled
            let mut glyphs = 0.0;
            let mut tahutis = 0.0;
            let mut bancrofts = 0.0;
            let mut executioners = 0.0;
            let mut sphinx_bool = false;

            for i in permutation
            {
                for j in &required_items
                {
                    if i.name==j.name {duplicate_detected_in_build = true;}
                    if j.name == "Bancroft's Talon" && !bancroft_enabled { bancroft_passive_lifesteal = 0.1; }
                }
                
                if i.name == BOOK_OF_THOTH.name && !book_of_thoth_enabled { duplicate_detected_in_build = true;}
                if i.name == WARLOCKS.name && !warlocks_enabled { duplicate_detected_in_build = true;}
                if i.name == TRANS.name && !trans_enabled { duplicate_detected_in_build = true;}
                if i.name == DEVOS.name && !devos_enabled { duplicate_detected_in_build = true;}
                if i.name == TABLET.name && !tablet_enabled { duplicate_detected_in_build = true;}                
                if (i.name == BANCROFTS.name || i.name == NIMBLE_BANCROFTS.name) && bancroft_enabled { bancroft_passive_lifesteal = 0.1; }

                if i.name == TAHUTI.name || i.name == CALAM_TAHUTI.name { tahutis += 1.0; }
                if i.name == BANCROFTS.name || i.name == NIMBLE_BANCROFTS.name || i.name == BANCROFTS_CLAW.name { bancrofts += 1.0; }
                if i.name == EXE.name || i.name == HEAVY_EXE.name || i.name == FEROCIOUS_EXE.name { executioners += 1.0; }
                if i.name == CALAM_TAHUTI.name || i.name == NIMBLE_BANCROFTS.name || i.name == BANCROFTS_CLAW.name || i.name == HEAVY_EXE.name || i.name == FEROCIOUS_EXE.name { glyphs += 1.0; }

                if i.name == SPHINX.name { sphinx_bool = true; }

                if god.class == "Hunter" && (i.name == SERRATED.name||i.name==SHADOWDRINKER.name) { duplicate_detected_in_build = true; }
                if god.class != "Hunter" && (i.name == FAILNOT.name) { duplicate_detected_in_build = true; }
                if god.name != RAT.name && (i.name == BRISTLEBUSH.name || i.name == THISTLETHORN.name || i.name ==  EVERGREEN.name || i.name == THICKBARK.name) { duplicate_detected_in_build = true;}

                build_vec.push(i);
            }

            if glyphs > 1.0 || tahutis > 1.0 || bancrofts > 1.0 || executioners > 1.0 { duplicate_detected_in_build = true; }

            let build_array:[&Item;6] = build_vec[..].try_into().unwrap();
            let build = combine_items(build_array);

            let mut antiheal_pass = true;
            if require_antiheal
            {
                antiheal_pass = false;
                for i in build.names.iter()
                {
                    if [BRAWLERS.name,TOXICBLADE.name,DIVINE.name,TAINTED_AMULET.name,TAINTED_STEEL.name,TAINTED_BREASTPLATE.name].contains(i){ antiheal_pass = true; }
                }
            }
            
            let mut cdr_after_cap = build.cdr + ((god.class=="Warrior")as i32 as f32) * (0.05 + 0.0025 * level);
            if sphinx_bool { cdr_after_cap = clamp(cdr_after_cap,0.0,0.5); }
            else { cdr_after_cap = clamp(cdr_after_cap,0.0,0.4); }

            if !duplicate_detected_in_build && cdr_after_cap >= min_cdr && build.lifesteal + bancroft_passive_lifesteal >= min_lifesteal && antiheal_pass && build.gold < max_gold
            {            

                let mut display_string = String::new();
                let mut build_fits_criteria = true;
                let mut target1_ability_damage = 0.0;
                let mut target2_ability_damage = 0.0;
                let mut target1_auto_damage = 0.0;
                let mut target2_auto_damage = 0.0;

                if calculate_ability_damage
                {
                    target1_ability_damage = ability_damage(base_ability_damage, ability_scaling, 
                        god, level, &build, target1_god, target1_level, &target1_build, obby_enabled, soul_gem_enabled, 
                        poly_enabled,estaff_enabled,conduit_enabled,archmages_enabled,focus_enabled,magus_enabled,doom_enabled,
                        binding_enabled,bancroft_enabled,tahuti_enabled,magical,
                        titans_bane_enabled,arondight_enabled,serrated_enabled,runeforged_enabled,cowl_enabled,
                        redstone_attack_speed_enabled,bumbas_spear_enabled,protector_enabled,mhb_enabled,ability_crit_enabled,ability_true_damage,hydras_enabled,
                        as_buff,sol_passive,flat_power_buff,percent_power_buff,red_buff,
                        purp_buff,fire_giant,enhanced_fire_giant,p500_pot,pen_pot,target1_mitigations,calam_enabled,claw_enabled,myr_enabled,
                        number_of_abilities,divine_enabled,boomerang_enabled);

                    if target1_ability_damage < target1_min_ability_damage{ build_fits_criteria = false;}
                    
                    else{

                        target2_ability_damage = ability_damage(base_ability_damage, ability_scaling, 
                            god, level, &build, target2_god, target2_level, &target2_build, obby_enabled, soul_gem_enabled, 
                            poly_enabled,estaff_enabled,conduit_enabled,archmages_enabled,focus_enabled,magus_enabled,doom_enabled,
                            binding_enabled,bancroft_enabled,tahuti_enabled,magical,
                            titans_bane_enabled,arondight_enabled,serrated_enabled,runeforged_enabled,cowl_enabled,
                            redstone_attack_speed_enabled,bumbas_spear_enabled,protector_enabled,mhb_enabled,ability_crit_enabled,ability_true_damage,hydras_enabled
                            ,as_buff,sol_passive,flat_power_buff,percent_power_buff,red_buff,
                            purp_buff,fire_giant,enhanced_fire_giant,p500_pot,pen_pot,target2_mitigations,calam_enabled,claw_enabled,myr_enabled,
                            number_of_abilities,divine_enabled,boomerang_enabled);

                        if target2_ability_damage < target2_min_ability_damage{ build_fits_criteria = false;}
                        
                        if !ttk_display
                        {
                            display_string.push_str("Ability Damage: Target 1 = ");
                            display_string.push_str(&(((target1_ability_damage*100.0).round())/100.0).to_string());
                            display_string.push_str(",   ");

                            display_string.push_str("Target 2 = ");
                            display_string.push_str(&(((target2_ability_damage*100.0).round())/100.0).to_string());
                            display_string.push_str("\n");
                        }

                        else
                        {
                            let target1_health = target1_god.base_health + (target1_god.health_per_level * (target1_level - 1.0)) + target1_build.health;
                            display_string.push_str("Ability Damage: Target 1 = ");
                            display_string.push_str(&(((target1_ability_damage / target1_health  * 10000.0).round())/100.0).to_string());
                            display_string.push_str("%,   ");

                            let target2_health = target2_god.base_health + (target2_god.health_per_level * (target2_level - 1.0)) + target2_build.health;
                            display_string.push_str("Target 2 = ");
                            display_string.push_str(&(((target2_ability_damage / target2_health * 10000.0).round())/100.0).to_string());
                            display_string.push_str("%\n");
                        }
                    }
                }

                if calculate_auto_dps && build_fits_criteria
                {
                    target1_auto_damage = auto_attack_dps(auto_sample_time, god, level, &build, target1_god, target1_level, &target1_build, temper_enabled,
                        diamond_enabled,focus_enabled,magus_enabled,doom_enabled,binding_enabled,bancroft_enabled,tahuti_enabled,magical,
                        serrated_enabled,runeforged_enabled,cowl_enabled,redstone_attack_speed_enabled,bumbas_spear_enabled,
                        protector_enabled,kaldr_enabled,pos2_enabled,as_buff,freya_1_2_enabled,baka_3_enabled,sol_passive,obby_enabled,
                        flat_power_buff,percent_power_buff,red_buff,purp_buff,fire_giant,enhanced_fire_giant,
                        p500_pot,pen_pot,target1_mitigations,chronos_2,ao_2,ferocious_enabled,myr_enabled,ishtar_1a,ishtar_1b,ishtar_1c,boomerang_enabled,
                        vital_enabled,spectral_aura_enabled);
                        
                    if target1_auto_damage < target1_min_auto_dps{ build_fits_criteria = false;}
                    
                    else
                    {
                        target2_auto_damage = auto_attack_dps(auto_sample_time, god, level, &build, target2_god, target2_level, &target2_build, temper_enabled,
                            diamond_enabled,focus_enabled,magus_enabled,doom_enabled,binding_enabled,bancroft_enabled,tahuti_enabled,magical,
                            serrated_enabled,runeforged_enabled,cowl_enabled,redstone_attack_speed_enabled,bumbas_spear_enabled,
                            protector_enabled,kaldr_enabled,pos2_enabled,as_buff,freya_1_2_enabled,baka_3_enabled,sol_passive,obby_enabled,
                            flat_power_buff,percent_power_buff,red_buff,purp_buff,fire_giant,enhanced_fire_giant,
                            p500_pot,pen_pot,target2_mitigations,chronos_2,ao_2,ferocious_enabled,myr_enabled,ishtar_1a,ishtar_1b,ishtar_1c,boomerang_enabled,
                            vital_enabled,spectral_aura_enabled);

                        if target2_auto_damage < target2_min_auto_dps{ build_fits_criteria = false;}
                              
                        if !ttk_display
                        {
                            display_string.push_str("Auto DPS:       Target 1 = ");
                            display_string.push_str(&(((target1_auto_damage*100.0).round())/100.0).to_string());
                            display_string.push_str(",   ");

                            display_string.push_str("Target 2 = ");
                            display_string.push_str(&(((target2_auto_damage*100.0).round())/100.0).to_string());
                        }

                        else
                        {
                            let target1_health = target1_god.base_health + (target1_god.health_per_level * (target1_level - 1.0)) + target1_build.health;
                            display_string.push_str("Auto DPS:       Target 1 = ");
                            display_string.push_str(&(((target1_health / target1_auto_damage * 100.0).round())/100.0).to_string());
                            display_string.push_str("s,   ");

                            let target2_health = target2_god.base_health + (target2_god.health_per_level * (target2_level - 1.0)) + target2_build.health;
                            display_string.push_str("Target 2 = ");
                            display_string.push_str(&(((target2_health / target2_auto_damage * 100.0).round())/100.0).to_string());
                            display_string.push_str("s");
                        }
                    }
                }
                
                if build_fits_criteria
                {   
                    if sort_mode == 0
                    {
                        println!("{}, {}, {}, {}, {}, {}",build.names[0],build.names[1],build.names[2],build.names[3],build.names[4],build.names[5]);
                        println!("{}",display_string);
                        println!{"Gold: {}", build.gold};
                        println!("");
                    }
                    else
                    {
                        let mut stored_display_string = String::new();
                        for name in build.names.iter() { stored_display_string.push_str(name); stored_display_string.push_str(", ");}
                        stored_display_string.push_str("\n");
                        stored_display_string.push_str(&display_string);
                        stored_display_string.push_str("\nGold: ");
                        stored_display_string.push_str(build.gold.to_string().as_str());
                        stored_display_string.push_str("\n\n");
                        
                        let array = [stored_display_string,target1_ability_damage.to_string(),target1_auto_damage.to_string(),target2_ability_damage.to_string(),target2_auto_damage.to_string()];
                        let damage = &array[sort_mode].parse::<f32>().unwrap();
                        
                        if damage > &sorted_builds[1][sort_mode].parse::<f32>().unwrap() && sorted_builds.len() == builds_to_store_with_sort + 2
                        {
                            sorted_builds.remove(1);
                        }

                        if sorted_builds.len() < builds_to_store_with_sort + 2
                        {
                            let mut sorted = false;
                            let mut current_index = (((sorted_builds.len()-1)/2) as f32).round() as usize;                       

                            while !sorted
                            {                                
                                if  damage <= &sorted_builds[current_index][sort_mode].parse::<f32>().unwrap()
                                {     
                                    if damage >= &sorted_builds[current_index-1][sort_mode].parse::<f32>().unwrap() { sorted = true;}                   
                                    else { current_index -= 1; }
                                }
                                else{ current_index += 1; }  
                            }                        

                            sorted_builds.insert(current_index,array)
                        }
                            
                        
                    }
                }
            }
            
        }

        sorted_builds.pop();
        sorted_builds.remove(0);

        if sort_mode != 0
        {
            for i in sorted_builds
            {
                println!("{}",i[0]);
            }
        }

        println!("\n\n\n");
    }
}



