# Development Log for CS4099 - Virutal Modular Synthesizer
## 24th Sept, 2022
Beginning devlog here, to keep track of design decisions made along the way.  So far:
- Using Rust + WASM to develop the audio engine, for mostly performance reasons, and I like Rust.  C++ is the normal choice for audio development, with mature package support, but the process of getting it to play nice with WASM seems like a PITA, and rust is pretty straightforward.  While the native JS implementations of the nodes I will be implementing are very fast, the nodes provided by the Web Audio API are fairly limitted.
- Using NextJS for the frontend, mostly because I'm comfortable with it.  Planning on just doing regular html elements for the nodes, and connecting them with generated SVG Bezier curves.  This is in opposition to doing a full canvas-based application.  Canvas based applications tend to be pretty laggy, in my experience.
- As far as influences goes, Glicol is a really interesting project, and does something pretty similar to this, with a powerful synthesis engine based in WASM.  The primary decision is that Glicol is aiming at building a DSP/Synthesis language, instead of a graphical tool.  The engine is open source, but from the looks of it the graph implementation isn't quite what I'm looking for, so I plan on rolling my own. There's also AudioNodes, which is a browser based modular synth similar to what I'm aiming for.  Unfortunately, the canvas based frontend is pretty laggy, and development seems fairly innactive.  In addition, it doesn't leverage WASM, one of the primary goals of this project.  Because of this, all of the audio processing gets done in Javascript.
- I'll be using the dasp crate for a lot of the audio processing basics.  However, the dasp_graph crate, which provides the audio graph implementation in dasp, is too limited for my implementation, so I have forked it in the audio_graph crate.  I will be adding the features I need there, as well as the node implementations I will need.  I got the idea for expanding upon dasp_graph from Glicol, although it takes it in a different direction.  The main change I'm making is to make it so instead of each node having arbitrary inputs and outputs, there will be many inputs and outputs with port numbers, on which each node can send and receive data.  One consequence of this is that all data will move at the sample rate.  If this leads to performance problems, it would be possible to make multiple kinds of connections, some which send full buffers, and others single values.
## 25th Sept, 2022
Been feeling a bit "in the hole" doing so much work on this.  Going to try and work at a more reasonable (and maintainable) pace.
- Continued refactoring the audio_graph fork.  Each `Node` has a function called `get_port` which is just a lookup using the port type and input name to the port number.  This will help keep magic port numbers out of the code, and hopefully will mean the port numbers only get used explicitly within each node's implementation.  The lookups should also be pretty fast.  I deleted the default `Signal` and `Sum` nodes, as I will be rewriting these from scratch anyways in some sort of `Oscillator` and `Mixer` nodes, respectively.  I'm fairly happy with how the audio_graph fork is coming together, I think it'll be successful at providing a clean interface for the more business logic-ey code of manipulating the nodes.  Of course, this is all as of yet untested, so we'll see how it goes :).  Tomorrow I'll do the impl for the subgraph node, which shouldn't be too bad but is non-trivial to refactor.  Subgraphs would be really nice, because modules could be encapsulated and shared around easily.
## 26th Sept, 2022
- Subgraph node implemented, was sort of a pain in the ass.  As it stands, it takes in a graph, which needs to have a single output node.  Then the user can create and map external input nodes to input nodes of the internal nodes of the subgraph.  For the output ports, the output ports from the internal output node are mirrored.
## 28th Sept, 2022
- Audio graph is working!
- Next step is writing an oscilloscope to make it easier to debug issues with audio modules.  A proof of concept was written in a code pen.  It uses analyzer, but we will just use the samples directly out of wasm.
## 30th Sept, 2022
- Made a crude oscilloscope to help with debugging.  Not my finest work but should do alright for now.
- Next step is to expand the oscillator module.   Would be good to get some sort of FM Synthesis test done ASAP.
## 1st Oct, 2022
- Setup messaging from the backend to the frontend.  The flow should keep the backend graph as the single source of truth.  Event flow should go something like this:
    - Frontend requests an action from audio engine
    - Audio engine validates and performs the action
    - Audio engine messages the frontend with the result of the action
    - Frontend performs visual update as a response.
- Added some convienience functions to the audio manager for creating/connecting nodes.
- Wanted to improve oscillator module but didn't have a way of messaging between the front and backend, now we do.
## 2nd Oct, 2022
- Started drawing components onto the frontend.  Creates the components on creation messages from the engine.  Currently do not have ports setup.
- Next step is to draw the ports.  I don't have time to do it today, but the idea is that each port will register itself upon first render, and unregister upon unmounting.  Registration will be in the form of the node ID, port ID, and a ref to the object.  This will all get stored in a context.  Some SVG drawer component, which will basically sit on top of the ModArea, will be able to get refs to all of the ports by their node ID and port ID, and then grab their x/y coordinates relative to the parent div, drawing the appropriate lines.
## 4th Oct, 2022
- Added ports along with their styling.  Made some wrapper to make it easier to create module frontends.  Should maybe be refactored.
## 6th Oct, 2022
- Been working on PLDI a lot, so less time for this.
- Got path drawing working!  It's kinda a small thing but I'm fairly happy with it.
- Hover effects added for ports/connections.

## 7th Oct, 2022
- Dragging works :) :) :)
- It's a bit buggy though, currently you can drag modules outside of the module area wrapper div.  I think I need to attach the mouse listener to that instead of the window.  Needs some refining.
- Path drawing is also a bit clunky and needs to be refined, but I'm going to put that in the "Polish" bucket, and deal with it later.

# 11th Oct
- Took a bit of a break to focus on PLDI, now back to it.
- Refactored the oscillator module to use custom DSP code instead of dasp::signal.  It's pretty simple, a phase accumulator keeps track of where in radians the phase of the oscillator is (calculated using the sample rate and the pitch), and then applies the appropriate function to the phase to get the next sample.  Write now I have the sine and square functions written.
- Message passing from frontend to backend implemented, the only message currently available is to change the parameter of some node.  While message passing works, the seam between the JS and the Rust is pretty rough.  I'd like to clean it up in the future if possible.
- Used the new message passing to implement a pitch slider for the oscillator module, This represents the first user control of the synth in the project.
- Planning on making users able to edit connections next.

# 12th Oct
- Big day today
- Connection editing/modifying implemented.  Users can now add connections between modules as they see fit.  One thing that doesn't work quite right is that the system does not restrict users from adding multiple patch cords to an input, which doesn't do anything as it stands.  This behaviour should require a mixer module, because it's confusing otherwise.
- FM Synthesis is possible! Granted, it's basically the only thing that works right now, but it's still pretty cool.
- Module addition implmented, you can add as many oscillators as your CPU lets you :).
- In order to get connections working, I needed to switch to using petgraph's StableGraph, as it keeps edge indexes stable upon deletion.  I assumed this would be trivial, but petgraph hadn't implemented the edges_connecting (which returns an iterator of edges between the two nodes).  To ammend this, I forked petgraph and added the functionality.  I should be able to make a PR for it in the near future.
- I need to figure out a better way of handling the normalized inputs to the pitch.  Ideally, +1 would pitch up the frequency by 128 midi notes, and vice versa for -1.
- I think I will have a pitch input for my oscillators, that directly controls the pitch, as well as a coarse and fine detune knob that work on semitones and cents respectively.  This will be a lot more flexible than the single pitch knob I use right now.
- Next step module-wise is probably a step sequencer module, then an EG.

# 24th Oct
- Took a break from things for a bit to focus on other practicals
- Built out a Math module, which in its current form is a 4 way attenuverting mixer.  It's a bit based on the Maths module by Make Noise but isn't anywhere near that feature set yet, much more to be added.  It's an important utility module as it allows me to mix signals easily.
- Having a demo with Ian tomorrow which should be fun.
- While implementing the module, there were a lot of moving parts that had to come together.  I'd like, if possible, to specify the module entirely in Rust, and just have the frontend be able to figure out how to render it automatically.  This will of course lead to some restrictions, in frontend design, but I think I'm willing to sacrifice that for ease of use.  A refactor to this is due.

# 25th Oct
- Some notes on the specification language for the modules:
    - The specification for a module will be a JSON object as follows:
    ```json
    module: {
        name: string
        rows: Row[]
    }
    Row: {
        input: Port | undefined
        output: Port | undefined
        elements: RowElement[]
    }
    Port: {
        name: string
        id: number 
    }
    RowElement: Text | Slider | Selector
    Text: {
        data: string 
    }
    Slider: {
        max: number
        min: number
        default: number
        parameter: string
    }
    Selector: {
        values: string[]
        parameter: string
    }
    ```
- Refactor has been made, making it much easier to specify the modules.  It does constrain design a bit but I'm not too concerned, as it's easily extendable.  It mostly allows me to think about all the connections a lot less.
- Still need to make the "add module" buttons respond to what the backend reports as the module specifications.
- Added lots more controls with my new powers.  Oscillator now has 4 types of wave, Sine, Square, Saw, Tri.  Controls now include pitch, coarse pitch offset, and fine pitch offset.  The base pitch is currently not modulatable.  I need to come up with some sort of standardized way of doing pitch information.  Probably will still the 1V/Oct standard from eurorack and just do .1CV/Oct or something.
- Still want to add more controls to the oscillator, namely, pulse width modulation and phase modulation.
- Right now the oscillator can go well into sub audio rate, but I still think it would be nice to have a dedicated LFO module, because it'd make things nicer.
- Would like to add slewing to some of the Math module inputs
- Next module will be a step sequencer, so I can get something semi-musical out of it!  I have some fun ideas, but for now just tempo, with 8 steps and pitch controls should be good.  Hence the need for standardization of pitch information.
- Also met with Ian today, he seemed impressed at the current functionality and performance, which I'm happy about.S

# 31 Oct
- Lots of deadlines done.  Added a phase input.  Nice to see that feeding back a sine wave with an input of 1 actually gives me white noise.

# 1 Nov
- Added an LFO Module, sample and hold should be next.  Also some sort of slew limiter on the math module inputs 1 and 2.

# 8th Nov
- Added a S&H module.  Want to add quantization, but allows for some simple (completely untuned) melodies to be played.  You can also hook the output up to multiple offset oscillators and it'll play chords for you.  Sorta fun.
- Added PolyBLEP to my discontinuous oscillators (except triangle, but I'm not too concerned about it).  I think something like MinBLEP would be better but it's a much trickier implementation.

# 21st Nov
- Made mappings of knobs nicer using some arcane math.
- Haven't had a ton of time owing to deadlines.
- Discovered a bug when too many entities are spawned.  The WASM memory buffer needs to grow, which works fine, but then the old pointers Javascript has are invalidated.  Need to figure out some sort of solution to this, although it seems tricky.