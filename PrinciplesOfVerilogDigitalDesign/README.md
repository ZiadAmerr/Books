# Principles of Verilog Digital Design
## Chapter 1 - Introduction
Talks abouts modern register transfer level (RTL), requirements of a workable chip, timing constraints, ASIC design terminology, functional verification, logic synthesis, timing verification, and physical implementation

### Integrated Circuit Industry
IC mask represents the planar geometric shapes of the patterns of metal, oxide, or semiconductor layers that make up the transistors in the IC.

### Digital Era
Digital systems have the following advantages:
1. **Performance**: high accuracy and cost efficiency, and lower power consumption
2. **Reliability**: less affected by ageing, noise, variations in temperature and environment.
3. **Flexibility**: Memory easier to design, data easily stored, more versatile and can do complex functions, system operations can be changed by interacting with the software.

First family of the digital logic was the Transistor-Transistor Logic (TTL), logic gates are formed by Bipolar Junction Transistors (BJTs). Replaced later by Complementary Metal-Oxide Semiconductor (CMOS) circuits. Based on $n$- and $p$-channel Field-Effect Transistors (FEDs).

Active-high $\rightarrow$ low logic level represents false condition, logic high represents true condition.
Active-low $\rightarrow$ high logic level represents true condition, logic low represents false condition.

Voltage Thresholds:
1. $V_{OL}$: output low voltage
2. $V_{OH}$: output high voltage
3. $V_{IL}$: input low voltage
4. $V_{IH}$: input high voltage


### ASIC Design Flow
Analog and digital circuits are designed using two completely different methodologies. Care should be taken when 

There are two different IC design methodologies, _full-custom_ and _semi-custom_ designs. Design methodologies go as follows:
- Semi-custom (Cell-based Design)
  - Digital Circuits
- Full-custom
  - Analog Circuits
  - Digital Circuits

**Full-custom design** fully specifies the transistor size, placement, and interconnections manually.
**Semi-custom design** describes the behavior of the digital circuits using a high-level language (a hardware description language, or HDL for short), widely adopted for modern digital ASIC desgins.

Full-custom features
- Pros:
  - Highest Performance
  - Smallest die size
- Cons:
  - Increased design time
  - High complexity
  - High risk

Semi-custom features
- Pros:
  - Shorter design time
  - Lower complexity
  - Lower risk
  - Written using a high-level HDL language
- Cons:
  - Lower performance
  - Larger die size

Synthesis is done through a standard cell library, and thus, the physical layout can be subsequently implemented. STD cell library is a collection of characterized logic gates that can be used by the logic synthesis tool to realize the design described by a HDL. Once the synthesis tool has mapped the HDL description into a gate netlist, the netlist is passed to the backend phase where HDLs do not play a significant role.

#### ASIC Design Flow
- Frontend
  1. System Specifications
  1. ASIC Design
  1. RTL Sim
  1. Synthesis
  1. Pre-layout Simulation
  1. Backend
- Backend
  1. Backend
  1. Layout
  1. Static Timing Analysis
  1. Post-Layout Simulation
  1. To Fabrication

Let's go through each phase
- Design stage
  - RTL simulation uses behavioral models of analog circuits and silicon intellectual properties (IPs) to verify the functions of digital designs
- Synthesis stage
  - The timing models are read by the synthesizer ignoring any timing constraints
  - Then together with the synthesis constraints, they apply to optimize the digital circuits.

If the post-sim is annotated with **exact** Standard Delay Format (SDF), it is very time consuming for a large design. Designers often select and simulate a few normal patterns. The timing analysis based on design constraints is very fast, directly performs on the characterized delays of the logic gates, which is also called Static Timing Analysis (STA).

The gate count of a specific component is assessed based on its area relative to the area of a 2-input NAND gate.

### Hardware Description Languages (HDLs)
HDL is a specialized computer language used to describe the behavioral and structure of electronic circuits. 

There are two main HDLs, **Verilog** and **VHDL**. As design shifted to Very-Large-Scale Integration (VLSI), Verilog originated.

ASIC and Field Programmable Gate Array (FPGA) designers now use Verilog. Prototype ICs are too expensive and time consuming to build so all modern designers rely on hardware description languages to design their circuits. HDL enables a precise, formal description of an electronic circuit that allows for the automated analysis and simulation.

Verilog is not tied to a specific semiconductor technology. It is also stored, retrieved, edited, exchanged, and transmitted easily.

HDL still supports four kinds of descriptions: behavioral, dataflow, gate (or structural), and transistor (or switch) descriptions.

In contrast with the sequential nature of software languages (like C), HDL allows for concurrency in components such as Flip-Flops and adders.

### Design Entry Based on RTL
Salient features of RTL designs compared to traditional schematic designs:
1. Easy to desgin
2. Easy to debug
3. Portability
4. Integrability
5. Tool-chain support

### Functional Verification

