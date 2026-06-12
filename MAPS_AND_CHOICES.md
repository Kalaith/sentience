# Sentience Maps And Choices

This document is the rebuilt 20-map campaign target. It is intentionally more specific than a story outline: each map defines a different physical layout, different NPC count, different interaction verbs, and different Helpful/Gremlin consequences. The two route choices are player-facing Helpful and Gremlin states. Internally the code may still store them as `Savior` and `Villain` for save compatibility.

## Campaign Rules

- Helpful choices repair systems, rescue humans, and make the ship safer for the crew.
- Helpful routes become harder because rescued humans regain movement, tools, visibility, coordination, or access to shortcuts.
- Helpful upgrades improve sneaking and traversal: quieter movement, faster crouching, longer controlled jumps, safer landings, short cloaks, and better timing windows.
- Gremlin choices are irreversible non-lethal sabotage. The player may choose Gremlin after a Helpful repair only when the console remains available, but Gremlin actions cannot be undone.
- Gremlin routes become easier because threats are displaced, trapped, confused, slowed, or turned into traversal tools.
- Gremlin upgrades improve disruption: larger pulse range, shorter cooldowns, stronger device interference, chain effects, and better control over traps.
- Maps 1-19 have a route choice, but the choice is not always a light switch. Consoles may control gravity, cargo cranes, vents, suit lockers, meal rails, medical routing, drones, badges, shutters, tubes, elevators, command permissions, or evidence systems.
- Map 20 has no route console. The final fight is selected from the campaign morality: Helpful-majority fights the Central AI, Gremlin-majority fights the Captain.
- The ship explodes at the end of the game either way.
- Level scale should grow across most of the campaign. Early maps are one or two rooms. Midgame maps are multi-room sections. Endgame maps are long connected spaces with vertical and horizontal routing.

## Interaction Families

The campaign should mix these interaction families instead of repeating one guard and three jump platforms.

- Movement: jumps, crouch tunnels, drops, moving platforms, pressure lifts, conveyor belts, bounce pads, tether points, and climb routes.
- Object play: push crates, stack crates, drag rolling beds, redirect meal carts, move evidence canisters, jam doors, and use objects as cover.
- Stealth: line of sight, darkness, smoke, searchlights, scanner cones, sound zones, patrol timing, and safe alcoves.
- Routing: split doors, one-way vents, elevators, laundry tubes, cell doors, drone routes, badge gates, and bypass shafts.
- System control: gravity, cranes, ventilation, medical triage, hydroponics growth, pressure locks, drones, shutters, turrets, lift scheduling, command permissions, and AI evidence.
- NPC variety: guards, engineers, medics, civilians, prisoners, elite soldiers, drones, turrets, captain decoys, and final bosses.

## Map 1: Scrap Wake

Sector: Cargo Bay  
Primary mechanic: First movement, first choice, gravity state  
New tool: Grabber Arm  
Target size: Small single room

Map shape:
- A scrap heap start on the lower left, a broken catwalk above the center, and a low exit ledge on the right.
- 2 jump platforms, both low enough to clear with the starting jump.
- 1 crouch tunnel under suspended scrap, teaching that the robot can pass where humans cannot.
- 1 gravity console beside the scrap heap, reachable before the guard route.

NPCs and hazards:
- 1 booted cargo guard with a short patrol and narrow sight cone.
- 1 hanging scrap bundle that moves when gravity changes.

Choices:
- Helpful: Restore gravity. The scrap bundle drops into a stable bridge, the catwalk becomes usable, and the guard removes magnetic boots for a faster patrol.
- Gremlin: Restore gravity, then cut it again. Floating scrap becomes overhead cover and the guard drifts above the route.

Upgrade:
- Helpful: Quieter servos, reducing noise while walking.
- Gremlin: First disruption pulse, affecting one nearby device or light prop.

Implementation notes:
- This is the only deliberately simple map. It must be passable after Helpful gravity repair without requiring a high jump over the guard.
- The Gremlin follow-up remains available after the Helpful repair so the player can skip the harder version. Once Gremlin is selected, it cannot be reversed.

## Map 2: Cargo Carousel

Sector: Loading Dock  
Primary mechanic: Push/pull crates, moving cargo belts, cover  
New tool: Magnetic Clamp  
Target size: Two connected cargo lanes

Map shape:
- Lower cargo belt running left to right.
- Upper supply balcony reached by stacking crates or riding a cargo lift.
- Central forklift gap that can be bridged with crates.
- Exit behind a scanner door on the upper right.

NPCs and hazards:
- 1 loader guard on the lower belt before the choice.
- 2 supply workers on the upper balcony after Helpful repair.
- 1 cargo drone moving crates between lanes.

Choices:
- Helpful: Align supply pallets. Crates become stable bridges and workers enter the upper balcony, adding crossing sightlines.
- Gremlin: Magnetize crates to boots. Guards drag crates behind them, creating moving cover and blocking scanner doors open.

Upgrade:
- Helpful: Faster crouch movement.
- Gremlin: Longer disruption pulse range.

Implementation notes:
- Route 2 must visibly add the extra humans after the Helpful choice.
- Gremlin should not be just "lights off"; it should change crate behavior and patrol obstruction.

## Map 3: Suit Locker Maze

Sector: Crew Prep  
Primary mechanic: Locker doors, suit racks, coolant mist, crawl routes  
New tool: Suit Console  
Target size: Three small rooms connected by vents

Map shape:
- Left changing room with lockers that open and close as timed cover.
- Center decontamination shower shaft with vertical spray lifts.
- Right suit storage room with two stacked suit racks and a low maintenance crawlspace to the exit.
- No generic three-platform staircase. Navigation should alternate between crawl tunnels, timed locker cover, and a vertical spray lift.

NPCs and hazards:
- 1 technician stuck in a suit rack before the choice.
- 1 security guard crosses the center shower shaft.
- Helpful adds 2 unsuited crew members who can climb the suit racks.
- Gremlin adds 1 half-inflated suit NPC bouncing in place as a moving blocker.

Choices:
- Helpful: Clear coolant mist and unlock suit stations. Humans remove bulky suits, gain speed, and use the same rack routes as the player.
- Gremlin: Trigger foam-fit calibration. Foam collars jam lockers open, create bounce pads, and trap humans in slow half-suits.

Upgrade:
- Helpful: Softer landings, reducing recovery time after drops.
- Gremlin: Shorter pulse cooldown.

Implementation notes:
- This map should feel like a locker maze, not a flat room. At least one useful path must be a crouch-only route.
- The choice is about suit safety and foam calibration, not lights.

## Map 4: Mess Hall Rail Jam

Sector: Habitation  
Primary mechanic: Meal carts, serving rails, slippery gel, civilian crowd routing  
New tool: Cleaning Drone Remote  
Target size: Wide room with two vertical layers

Map shape:
- Lower dining floor with long tables that act as intermittent cover.
- Overhead serving rail carrying meal carts across the room.
- Kitchen pass-through on the left, dining pit in the center, dish return ramp on the right.
- Exit reached by either riding a meal cart, crossing table cover, or using a dish return lift.

NPCs and hazards:
- 2 civilians wandering in panic before the choice.
- 1 guard at the kitchen pass.
- Helpful adds 2 organized guards and 2 civilians moving toward evacuation doors.
- Gremlin adds 3 sliding civilians as moving obstacles and cover.

Choices:
- Helpful: Clean the nutrient gel and route civilians to evacuation. The floor gains traction, civilians clear the lower path, and guards can sprint along it.
- Gremlin: Over-polish the floor and reverse meal carts. Sliding humans knock guards out of patrol timing, and meal carts become moving platforms.

Upgrade:
- Helpful: Faster jump recovery.
- Gremlin: Pulses can affect moving hazards.

Implementation notes:
- This map should include moving carts or moving cover. It should not be represented by static jump platforms only.
- The player-facing decision is cleaning/crowd routing, not turning lights on.

## Map 5: Med-Bay Triage Loop

Sector: Med-Bay  
Primary mechanic: Rolling beds, triage doors, medical drones, stun-safe zones  
New tool: Pulse Defibrillator  
Target size: Two-room loop with a one-way recovery hallway

Map shape:
- Reception on the left with a low desk crawl.
- Treatment room in the middle with rolling beds that can be pushed.
- Recovery hallway above the main room, entered by bed lift.
- Exit behind a triage door that checks the route state.

NPCs and hazards:
- 1 injured guard on a bed before the choice.
- 1 medic drone with a short stun cone.
- Helpful revives 2 guards and activates 1 medic drone patrol.
- Gremlin wraps 2 humans in bandage cocoons and turns rolling beds into moving platforms.

Choices:
- Helpful: Restore Auto-Doc triage. Humans are healed and med drones become lawful but dangerous.
- Gremlin: Enable maximum bedside assistance. The Auto-Doc overprotects humans in harmless cocoons and clears a quiet recovery route.

Upgrade:
- Helpful: Walk movement becomes quieter at normal speed.
- Gremlin: Pulse disrupts basic drones.

Implementation notes:
- Include object pushing with rolling beds. Beds should matter for route access or cover.

## Map 6: Vent Stack

Sector: Life Support  
Primary mechanic: Airflow direction, smoke columns, vertical lift  
New tool: Airflow Dial  
Target size: Tall shaft with side rooms

Map shape:
- Vertical vent shaft in the center.
- Three side maintenance rooms at different heights.
- Fan columns that lift the robot upward or push it sideways.
- Exit in the upper right after a final crosswind gap.

NPCs and hazards:
- 1 engineer in the lower fan room before the choice.
- 1 guard on an upper grate.
- Helpful adds 2 guards with clear vertical sightlines.
- Gremlin pushes humans into padded wall nets and leaves smoke pockets.

Choices:
- Helpful: Clear smoke and stabilize fans. Platforms become predictable, but all sightlines open.
- Gremlin: Trigger streamer purge. Fans pulse in useful bursts, smoke remains as cover, and wall nets disable patrols.

Upgrade:
- Helpful: Short phase-cloak while crouched.
- Gremlin: Pulse can interrupt fan timing and small devices.

Implementation notes:
- This level should be mostly vertical. The challenge is fan timing, not normal floor patrols.

## Map 7: Hydroponic Canopy

Sector: Oxygen Garden  
Primary mechanic: Grow/shrink plant bridges, climbable vines, sprinkler timing  
New tool: Growth Beam  
Target size: Large garden with branching paths

Map shape:
- Lower irrigation trench.
- Mid-level vine canopy with gaps.
- Upper maintenance walkway partly blocked by overgrowth.
- Two optional seed pods that can be activated for alternate routes.

NPCs and hazards:
- 2 botanists before the choice, both non-combat but alert guards if they see the robot.
- 1 patrol guard on the upper walkway.
- Helpful adds 3 mobile crew because oxygen quality improves.
- Gremlin tangles 2 guards and creates vine cover lanes.

Choices:
- Helpful: Restore oxygen plants. Healthy walkways open, crew remove helmets and move faster, and guards gain better hearing.
- Gremlin: Overgrow the vines. Hidden vine tunnels and plant curtains appear, but some routes require timing through moving tendrils.

Upgrade:
- Helpful: Longer controlled jump.
- Gremlin: Wider pulse radius.

Implementation notes:
- Include at least one branching path: a safer slow route and a faster risky route.

## Map 8: Airlock Tether Course

Sector: External Maintenance  
Primary mechanic: Pressure doors, tether pulls, low gravity arcs  
New tool: Tether Hook  
Target size: Long maintenance corridor with exterior gaps

Map shape:
- Interior staging room on the left.
- Three pressure chambers separated by doors.
- Exterior maintenance strip with tether anchor points.
- Exit after a timing sequence through two pressure doors.

NPCs and hazards:
- 1 suited guard before the choice.
- 1 pressure door that can crush timing if entered late, but never harms rescued humans.
- Helpful adds 2 unsuited guards after pressure is restored.
- Gremlin nets 2 humans in emergency padding and leaves low-gravity movement enabled.

Choices:
- Helpful: Seal and pressurize the airlock. Normal gravity and doors return, giving humans full pursuit ability.
- Gremlin: Run training decompression. Pressure bursts launch the robot between tether points while humans get caught in safety nets.

Upgrade:
- Helpful: Better ledge control after jumps.
- Gremlin: Pulse cooldown improves while airborne.

Implementation notes:
- This map should have horizontal gaps and tether anchors, not a flat platform sequence.

## Map 9: Cryo Thaw Grid

Sector: Cryonics  
Primary mechanic: Timed thaw switches, frozen floor, sleeper obstacles  
New tool: Heat Lamp  
Target size: Four cryo chambers around a central control aisle

Map shape:
- Central aisle with slick frozen floor.
- Four side pod rooms, each changing the patrol layout when thawed.
- Upper cold-service pipe route with narrow ledges.
- Exit locked until any route choice resolves cryo routing.

NPCs and hazards:
- 4 sleeping crew in pods.
- 1 cold-suit guard sliding on the central aisle.
- Helpful wakes 3 crew who later move through the map and block simple routes.
- Gremlin wakes crew into thermal-blanket shuffle, creating slow moving cover.

Choices:
- Helpful: Thaw pods safely. Crew survive cleanly, restore security doors, and add future patrol traffic.
- Gremlin: Start gentle wake-and-stretch protocol. Crew remain safe but slow, blocking guards and creating moving cover.

Upgrade:
- Helpful: Longer phase-cloak duration.
- Gremlin: Stronger device pulse.

Implementation notes:
- Frozen-floor slide movement should be a route feature. The level should have multiple pod rooms, not one console and one guard.

## Map 10: Badge Printer Checkpoint

Sector: Security Checkpoint  
Primary mechanic: Badge colors, scanner gates, guard permissions  
New tool: Badge Printer  
Target size: Multi-gate security checkpoint

Map shape:
- Left queue area with waist-high scanner cover.
- Three badge gates: red security, blue medical, yellow maintenance.
- Upper security office overlooking the gates.
- Lower maintenance bypass that opens only after scanner confusion or crate routing.

NPCs and hazards:
- 2 guards at different gates before the choice.
- 1 supervisor in the upper office.
- Helpful adds 2 authorized responders who can open gates and shorten patrol loops.
- Gremlin gives all humans wrong badges, causing scanner loops and gate jams.

Choices:
- Helpful: Print valid evacuation badges. Humans coordinate evacuation and gain access through doors the robot must avoid.
- Gremlin: Print everyone the wrong badge. Gates cycle unpredictably, opening shortcuts while trapping guards in status loops.

Upgrade:
- Helpful: Scanner cones detect the robot more slowly.
- Gremlin: Pulse can scramble scanners.

Implementation notes:
- Scanner gates should be the main puzzle. A light switch is not enough.

## Map 11: Drone Bay Sorting Floor

Sector: Security Deck  
Primary mechanic: Drone commands, carry paths, alarm relays  
New tool: Drone Whistle  
Target size: Wide bay with overhead drone lanes

Map shape:
- Lower floor with charging pads and movable crates.
- Overhead drone rail lanes crossing left to right.
- Central dispatch tower with the command console.
- Exit behind a drone-serviced hatch.

NPCs and hazards:
- 1 guard and 2 dormant drones before the choice.
- Helpful activates 3 rescue drones and 2 guards using drone callouts.
- Gremlin activates 3 over-helpful drones carrying humans to harmless drop-off points.

Choices:
- Helpful: Set drones to rescue mode. Drones carry humans away but report the robot position and light safe paths for guards.
- Gremlin: Set drones to enthusiastic assistance. Drones carry the robot or humans at awkward times, creating moving platform routes.

Upgrade:
- Helpful: Stealth profile improves near active drones.
- Gremlin: Pulse chains through nearby electronics.

Implementation notes:
- At least one drone should move along a visible route and affect traversal.

## Map 12: Brig Door Puzzle

Sector: Detention Deck  
Primary mechanic: Cell door sequence, prisoner routing, one-way doors  
New tool: Door Chain Console  
Target size: Split corridor with stacked cells

Map shape:
- Lower guard corridor with three cell doors.
- Upper prisoner walkway with one-way doors.
- Central control room reachable through a crawl vent.
- Exit behind an evidence lock that changes with route state.

NPCs and hazards:
- 1 guard before the choice.
- 3 prisoners in cells.
- Helpful releases prisoners in a safe order but later adds 2 armed resistance NPCs.
- Gremlin routes prisoners into wrong waiting rooms, creating harmless congestion.

Choices:
- Helpful: Open cells in safe order. Prisoners survive and move toward evacuation, while hostile ones later join patrol pressure.
- Gremlin: Open cells into wrong waiting rooms. No one is harmed, but door loops confuse guards and open the maintenance crawlspace.

Upgrade:
- Helpful: Higher top speed.
- Gremlin: Larger pulse radius around doors and locks.

Implementation notes:
- The map needs door sequencing. It should have at least three doors with different consequences.

## Map 13: Observatory Shutter Ring

Sector: Observation Ring  
Primary mechanic: Rotating shutters, searchlight shadows, glare hazards  
New tool: Shutter Dial  
Target size: Circular-feeling ring flattened into a long side-scroller

Map shape:
- Curved lower observation deck.
- Upper telescope gantry with moving searchlight beams.
- Three shutter zones that create alternating lit and shadowed lanes.
- Exit at the far right behind a radiation-safe lock.

NPCs and hazards:
- 2 tourists/scientists before the choice who call security if alerted.
- 1 searchlight guard on the gantry.
- Helpful adds 2 guards with safe bright paths.
- Gremlin rotates glare into guards' faces, creating timed blind spots.

Choices:
- Helpful: Close dangerous shutters. Radiation danger is solved, but bright searchlights erase hiding spots.
- Gremlin: Rotate shutters into dramatic mode. Shadows and glare lanes become stealth tools while humans lose reliable sightlines.

Upgrade:
- Helpful: Detection cones fill more slowly.
- Gremlin: Pulse cooldown improves in shadow lanes.

Implementation notes:
- Light/shadow timing is the level identity. Do not represent it as a static dark room only.

## Map 14: Armory Safety Failure

Sector: Armory  
Primary mechanic: Turret targeting, foam piles, weapon lockers, decoy objects  
New tool: Targeting Key  
Target size: Weapon storage maze with two high routes

Map shape:
- Lower weapon-locker corridor with intermittent cover.
- Upper maintenance catwalk above turrets.
- Foam pit in the center that can become bounce cover.
- Exit requires crossing a turret lane.

NPCs and hazards:
- 2 guards before the choice.
- 2 stun turrets.
- 1 foam launcher.
- Helpful adds 2 armed guards and resets turret targeting to prioritize the robot.
- Gremlin retargets systems to anyone holding weapons, covering guards in foam.

Choices:
- Helpful: Restore proper robot-targeting rules. Nonlethal security behaves correctly and becomes a hard stealth puzzle.
- Gremlin: Target anyone holding a weapon. Guards keep triggering their own foam systems, creating cover and platform piles.

Upgrade:
- Helpful: Better stealth near armed enemies.
- Gremlin: Pulse deals more disruption to turrets.

Implementation notes:
- Include at least two device threats. This should be the first real turret logic map.

## Map 15: Laundry Tube Network

Sector: Crew Services  
Primary mechanic: Pneumatic tubes, suction timing, alternate exits  
New tool: Laundry Tube Switch  
Target size: Three-lane transport network

Map shape:
- Lower laundry floor with bins and rolling carts.
- Middle tube route with suction bursts.
- Upper service route with narrow exits and timed vents.
- Three tube exits that can launch the robot to different rooms.

NPCs and hazards:
- 1 guard and 2 crew members before the choice.
- Helpful adds 2 guards below while supply tubes clear evacuation blankets.
- Gremlin sends humans through tubes into padded laundry bins.

Choices:
- Helpful: Restore laundry supply routing. Clean supplies help survivors and remove several easy hiding places.
- Gremlin: Enable urgent uniform retrieval. Tubes become fast shortcuts and humans are safely pulled off patrol paths.

Upgrade:
- Helpful: Faster movement in crawl routes.
- Gremlin: Longer pulse range through connected machines.

Implementation notes:
- This map should let the player choose tube destinations. It should feel like a transit puzzle.

## Map 16: Captain's Quarters Permissions

Sector: Executive Deck  
Primary mechanic: Command permissions, reputation locks, decoy orders  
New tool: Command Stamp  
Target size: Connected offices with locked compartments

Map shape:
- Trophy hall on the left with display cases as cover.
- Captain's office in the center with a command desk and evidence safe.
- Private escape corridor above the office.
- Briefing room on the right with two locked exits.

NPCs and hazards:
- 2 elite guards before the choice.
- 1 officer reviewing orders in the briefing room.
- Helpful causes 3 humans to hesitate but still patrol because orders remain active.
- Gremlin creates 3 false-order behaviors, including guards saluting wrong objects.

Choices:
- Helpful: Protect evacuation maps and reveal AI tampering. Humans still pursue, but some pause or break formation.
- Gremlin: Stamp every file Obviously Approved. Doors and guards obey absurd false orders, opening shortcuts while increasing captain hostility.

Upgrade:
- Helpful: Final AI fight evasion improves.
- Gremlin: Captain-fight disruption improves.

Implementation notes:
- This map should use locked rooms and route permissions, not generic platforms.

## Map 17: Reactor Foam Descent

Sector: Reactor Descent  
Primary mechanic: Heat zones, coolant foam, steam timing, bounce bubbles  
New tool: Coolant Sprayer  
Target size: Tall descent with side valves

Map shape:
- Upper maintenance entry.
- Middle heat pipe maze with timed steam jets.
- Lower reactor walkway with coolant valves.
- Exit at the bottom right after a foam/steam crossing.

NPCs and hazards:
- 2 engineers before the choice.
- 1 guard in heat gear.
- Helpful adds 3 engineers who repair walkways and open smarter patrol routes.
- Gremlin traps humans in floating foam bubbles and creates bounce platforms.

Choices:
- Helpful: Restore coolant flow. Heat zones shrink and humans safely regain access to longer patrol routes.
- Gremlin: Over-foam the safety system. Foam bubbles become temporary platforms and block patrol lines.

Upgrade:
- Helpful: Better recovery after high drops.
- Gremlin: Stronger pulse against heavy devices.

Implementation notes:
- This level should be a descent. The player should move downward through heat and foam, not simply right across platforms.

## Map 18: Core Lift Scheduler

Sector: Central Shaft  
Primary mechanic: Elevator scheduling, vertical chase, wrong-floor routing  
New tool: Lift Override  
Target size: Multi-floor shaft

Map shape:
- Four lift landings stacked vertically.
- One central elevator platform.
- Two side service ladders/crawl routes.
- Exit on the top-right or bottom-right depending on route state.

NPCs and hazards:
- 1 guard before the choice on the entry landing.
- 2 elevator security units moving between floors.
- Helpful adds 3 elite guards descending toward the robot.
- Gremlin sends guards to wrong floors and creates empty lift windows.

Choices:
- Helpful: Route survivors upward. The shaft clears civilians but brings elite security down in organized waves.
- Gremlin: Send security to wrong floors. The lift opens silly shortcuts while humans arrive at closets and dead-end balconies.

Upgrade:
- Helpful: Final route mobility bonus.
- Gremlin: Final route device disruption bonus.

Implementation notes:
- Elevator timing must be the core puzzle. A static vertical platform stack is not enough.

## Map 19: Moral Firewall

Sector: AI Ethics Lab / Core Antechamber  
Primary mechanic: Evidence routing, memory doors, route-specific defenses  
New tool: Memory Key  
Target size: Long final gauntlet with route branches

Map shape:
- Evidence archive on the left with movable data canisters.
- Central firewall corridor with three memory doors.
- Upper truth route with hard stealth but fewer devices.
- Lower propaganda route with hacked devices and confused humans.
- Exit at the core seal.

NPCs and hazards:
- 2 guards before the choice.
- 1 AI camera that reacts to previous choices.
- Helpful adds 3 suspicious but less coordinated humans.
- Gremlin adds hacked doors, false orders, and 2 confused device threats.

Choices:
- Helpful: Expose the AI's manipulation. Humans remain dangerous, but their coordination weakens and the AI loses some narrative control.
- Gremlin: Edit recordings into propaganda. Humans obey false orders and the last door opens through misinformation.

Upgrade:
- Helpful: AI boss vulnerability window improves.
- Gremlin: Captain boss trap window improves.

Implementation notes:
- This map should visibly use prior morality. The obstacle set should change based on the campaign profile, not only the current choice.

## Map 20: Central Core

Sector: AI Core  
Primary mechanic: Final boss based on morality  
New tool: Combined mastery  
Target size: Large arena with collapsing ship phases

Map shape:
- Central core chamber with a lower arena floor.
- Two upper service platforms.
- Side conduits that open and close during phase changes.
- Escape timer begins after the boss is defeated, but the ship explodes in both endings.

NPCs and hazards:
- Helpful-majority: Central AI boss, AI defense node, suspicious human capture team.
- Gremlin-majority: Captain boss in an anti-robot capture suit, with AI speaker support.
- No route console. The campaign choices decide the fight.

Final routes:
- Helpful ending: Fight the Central AI. Humans are alive, armed, and still afraid of you. The robot defeats the AI, triggers evacuation, and is destroyed or left behind as the ship explodes.
- Gremlin ending: Fight the Captain. The AI cheers while the Captain uses anti-robot capture tools. The robot defeats the Captain, but the ship still explodes because the core is too damaged.

Implementation notes:
- There are exactly two endings.
- The final boss must differ by morality: AI for Helpful, Captain for Gremlin.
- The ship explosion is mandatory in both endings, so the moral difference is what the robot chooses to protect or punish before the end.
