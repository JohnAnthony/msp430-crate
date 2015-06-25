#[allow(dead_code)]

pub mod g2211 {
    // STANDARD BITS
    pub const BIT0: u16 = 0x0001;
    pub const BIT1: u16 = 0x0002;
    pub const BIT2: u16 = 0x0004;
    pub const BIT3: u16 = 0x0008;
    pub const BIT4: u16 = 0x0010;
    pub const BIT5: u16 = 0x0020;
    pub const BIT6: u16 = 0x0040;
    pub const BIT7: u16 = 0x0080;
    pub const BIT8: u16 = 0x0100;
    pub const BIT9: u16 = 0x0200;
    pub const BITA: u16 = 0x0400;
    pub const BITB: u16 = 0x0800;
    pub const BITC: u16 = 0x1000;
    pub const BITD: u16 = 0x2000;
    pub const BITE: u16 = 0x4000;
    pub const BITF: u16 = 0x8000;

    // STATUS REGISTER BITS
    pub const C: u16      = 0x0001;
    pub const Z: u16      = 0x0002;
    pub const N: u16      = 0x0004;
    pub const V: u16      = 0x0100;
    pub const GIE: u16    = 0x0008;
    pub const CPUOFF: u16 = 0x0010;
    pub const OSCOFF: u16 = 0x0020;
    pub const SCG0: u16   = 0x0040;
    pub const SCG1: u16   = 0x0080;

    // Low Power Modes coded with Bits 4-7 in SR
    pub const IE1_: u16  = 0x0000;  // Interrupt Enable 1
    pub const IE1: u16   = IE1_;    // Alias
    pub const WDTIE: u8  = 0x01;    // Watchdog Interrupt Enable
    pub const OFIE: u8   = 0x02;    // Osc. Fault  Interrupt Enable
    pub const NMIIE: u8  = 0x10;    // NMI Interrupt Enable
    pub const ACCVIE: u8 = 0x20;    // Flash Access Violation Interrupt Enable

    pub const IFG1_: u16 = 0x0002;  // Interrupt Flag 1
    pub const IFG1: u16 = IFG1_;    // Alias
    pub const WDTIFG: u8 = 0x01;    // Watchdog Interrupt Flag
    pub const OFIFG: u8 = 0x02;     // Osc. Fault Interrupt Flag
    pub const PORIFG: u8 = 0x04;    // Power On Interrupt Flag
    pub const RSTIFG: u8 = 0x08;    // Reset Interrupt Flag
    pub const NMIIFG: u8 = 0x10;    // NMI Interrupt Flag

    // Basic Clock Module
    pub const __MSP430_HAS_BC2__: bool = true;  // Show that Module is available
    pub const DCOCTL_: u16  = 0x0056;   // DCO Clock Frequency Control
    pub const DCOCTL: u16   = DCOCTL_;  // Alias
    pub const BCSCTL1_: u16 = 0x0057;   // Basic Clock System Control 1
    pub const BCSCTL1: u16  = BCSCTL1_; // Alias
    pub const BCSCTL2_: u16 = 0x0058;   // Basic Clock System Control 2
    pub const BCSCTL2: u16  = BCSCTL2_; // Alias
    pub const BCSCTL3_: u16 = 0x0053;   // Basic Clock System Control 3
    pub const BCSCTL3: u16  = BCSCTL3_; // Alias

    pub const MOD0: u16 = 0x01;  // Modulation Bit 0
    pub const MOD1: u16 = 0x02;  // Modulation Bit 1
    pub const MOD2: u16 = 0x04;  // Modulation Bit 2
    pub const MOD3: u16 = 0x08;  // Modulation Bit 3
    pub const MOD4: u16 = 0x10;  // Modulation Bit 4
    pub const DCO0: u16 = 0x20;  // DCO Select Bit 0
    pub const DCO1: u16 = 0x40;  // DCO Select Bit 1
    pub const DCO2: u16 = 0x80;  // DCO Select Bit 2

    pub const RSEL0: u16  = 0x01;  // Range Select Bit 0
    pub const RSEL1: u16  = 0x02;  // Range Select Bit 1
    pub const RSEL2: u16  = 0x04;  // Range Select Bit 2
    pub const RSEL3: u16  = 0x08;  // Range Select Bit 3
    pub const DIVA0: u16  = 0x10;  // ACLK Divider 0
    pub const DIVA1: u16  = 0x20;  // ACLK Divider 1
    pub const XTS: u16    = 0x40;  // LFXTCLK 0:Low Freq. / 1: High Freq.
    pub const XT2OFF: u16 = 0x80;  // Enable XT2CLK

    pub const DIVA_0: u16 = 0x00;  // ACLK Divider 0: /1
    pub const DIVA_1: u16 = 0x10;  // ACLK Divider 1: /2
    pub const DIVA_2: u16 = 0x20;  // ACLK Divider 2: /4
    pub const DIVA_3: u16 = 0x30;  // ACLK Divider 3: /8

    pub const DIVS0: u16 = 0x02;  // SMCLK Divider 0
    pub const DIVS1: u16 = 0x04;  // SMCLK Divider 1
    pub const SELS: u16  = 0x08;  // SMCLK Source Select 0:DCOCLK / 1:XT2CLK/LFXTCLK
    pub const DIVM0: u16 = 0x10;  // MCLK Divider 0
    pub const DIVM1: u16 = 0x20;  // MCLK Divider 1
    pub const SELM0: u16 = 0x40;  // MCLK Source Select 0
    pub const SELM1: u16 = 0x80;  // MCLK Source Select 1

    pub const DIVS_0: u16 = 0x00;  // SMCLK Divider 0: /1
    pub const DIVS_1: u16 = 0x02;  // SMCLK Divider 1: /2
    pub const DIVS_2: u16 = 0x04;  // SMCLK Divider 2: /4
    pub const DIVS_3: u16 = 0x06;  // SMCLK Divider 3: /8

    pub const DIVM_0: u16 = 0x00;  // MCLK Divider 0: /1
    pub const DIVM_1: u16 = 0x10;  // MCLK Divider 1: /2
    pub const DIVM_2: u16 = 0x20;  // MCLK Divider 2: /4
    pub const DIVM_3: u16 = 0x30;  // MCLK Divider 3: /8

    pub const SELM_0: u16 = 0x00;  // MCLK Source Select 0: DCOCLK
    pub const SELM_1: u16 = 0x40;  // MCLK Source Select 1: DCOCLK
    pub const SELM_2: u16 = 0x80;  // MCLK Source Select 2: XT2CLK/LFXTCLK
    pub const SELM_3: u16 = 0xC0;  // MCLK Source Select 3: LFXTCLK

    pub const LFXT1OF: u16 = 0x01;  // Low/high Frequency Oscillator Fault Flag
    pub const XT2OF: u16   = 0x02;  // High frequency oscillator 2 fault flag
    pub const XCAP0: u16   = 0x04;  // XIN/XOUT Cap 0
    pub const XCAP1: u16   = 0x08;  // XIN/XOUT Cap 1
    pub const LFXT1S0: u16 = 0x10;  // Mode 0 for LFXT1 (XTS = 0)
    pub const LFXT1S1: u16 = 0x20;  // Mode 1 for LFXT1 (XTS = 0)
    pub const XT2S0: u16   = 0x40;  // Mode 0 for XT2
    pub const XT2S1: u16   = 0x80;  // Mode 1 for XT2

    pub const XCAP_0: u16 = 0x00;  // XIN/XOUT Cap : 0 pF
    pub const XCAP_1: u16 = 0x04;  // XIN/XOUT Cap : 6 pF
    pub const XCAP_2: u16 = 0x08;  // XIN/XOUT Cap : 10 pF
    pub const XCAP_3: u16 = 0x0C;  // XIN/XOUT Cap : 12.5 pF

    pub const LFXT1S_0: u16 = 0x00;  // Mode 0 for LFXT1 : Normal operation
    pub const LFXT1S_1: u16 = 0x10;  // Mode 1 for LFXT1 : Reserved
    pub const LFXT1S_2: u16 = 0x20;  // Mode 2 for LFXT1 : VLO
    pub const LFXT1S_3: u16 = 0x30;  // Mode 3 for LFXT1 : Digital input signal

    pub const XT2S_0: u16 = 0x00;  // Mode 0 for XT2 : 0.4 - 1 MHz
    pub const XT2S_1: u16 = 0x40;  // Mode 1 for XT2 : 1 - 4 MHz
    pub const XT2S_2: u16 = 0x80;  // Mode 2 for XT2 : 2 - 16 MHz
    pub const XT2S_3: u16 = 0xC0;  // Mode 3 for XT2 : Digital input signal

    // Comparator A
    pub const __MSP430_HAS_CAPLUS__: bool = true; // Definition to show that Module is available
    pub const CACTL1_: u16 = 0x0059;   // Comparator A Control 1
    pub const CACTL1: u16  = CACTL1_;  // Alias
    pub const CACTL2_: u16 = 0x005A;   // Comparator A Control 2
    pub const CACTL2: u16  = CACTL2_;  // Alias
    pub const CAPD_: u16   = 0x005B;   // Comparator A Port Disable
    pub const CAPD: u16    = CAPD_;    // Alias

    pub const CAIFG: u16  = 0x01;  // Comp. A Interrupt Flag
    pub const CAIE: u16   = 0x02;  // Comp. A Interrupt Enable
    pub const CAIES: u16  = 0x04;  // Comp. A Int. Edge Select: 0:rising / 1:falling
    pub const CAON: u16   = 0x08;  // Comp. A enable
    pub const CAREF0: u16 = 0x10;  // Comp. A Internal Reference Select 0
    pub const CAREF1: u16 = 0x20;  // Comp. A Internal Reference Select 1
    pub const CARSEL: u16 = 0x40;  // Comp. A Internal Reference Enable
    pub const CAEX: u16   = 0x80;  // Comp. A Exchange Inputs

    pub const CAREF_0: u16 = 0x00;  // Comp. A Int. Ref. Select 0 : Off
    pub const CAREF_1: u16 = 0x10;  // Comp. A Int. Ref. Select 1 : 0.25*Vcc
    pub const CAREF_2: u16 = 0x20;  // Comp. A Int. Ref. Select 2 : 0.5*Vcc
    pub const CAREF_3: u16 = 0x30;  // Comp. A Int. Ref. Select 3 : Vt

    pub const CAOUT: u16   = 0x01;  // Comp. A Output
    pub const CAF: u16     = 0x02;  // Comp. A Enable Output Filter
    pub const P2CA0: u16   = 0x04;  // Comp. A +Terminal Multiplexer
    pub const P2CA1: u16   = 0x08;  // Comp. A -Terminal Multiplexer
    pub const P2CA2: u16   = 0x10;  // Comp. A -Terminal Multiplexer
    pub const P2CA3: u16   = 0x20;  // Comp. A -Terminal Multiplexer
    pub const P2CA4: u16   = 0x40;  // Comp. A +Terminal Multiplexer
    pub const CASHORT: u16 = 0x80;  // Comp. A Short + and - Terminals

    pub const CAPD0: u16 = 0x01;  // Comp. A Disable Input Buffer of Port Register .0
    pub const CAPD1: u16 = 0x02;  // Comp. A Disable Input Buffer of Port Register .1
    pub const CAPD2: u16 = 0x04;  // Comp. A Disable Input Buffer of Port Register .2
    pub const CAPD3: u16 = 0x08;  // Comp. A Disable Input Buffer of Port Register .3
    pub const CAPD4: u16 = 0x10;  // Comp. A Disable Input Buffer of Port Register .4
    pub const CAPD5: u16 = 0x20;  // Comp. A Disable Input Buffer of Port Register .5
    pub const CAPD6: u16 = 0x40;  // Comp. A Disable Input Buffer of Port Register .6
    pub const CAPD7: u16 = 0x80;  // Comp. A Disable Input Buffer of Port Register .7

    // Flash Memory
    pub const __MSP430_HAS_FLASH2__: bool = true; // Definition to show that Module is available
    pub const FCTL1_: u16 = 0x0128;  // FLASH Control 1
    pub const FCTL1: u16 = FCTL1_;   // Alias
    pub const FCTL2_: u16 = 0x012A;  // FLASH Control 2
    pub const FCTL2: u16 = FCTL2_;   // Alias
    pub const FCTL3_: u16 = 0x012C;  // FLASH Control 3
    pub const FCTL3: u16 = FCTL3_;   // Alias

    pub const FRKEY: u16 = 0x9600;  // Flash key returned by read
    pub const FWKEY: u16 = 0xA500;  // Flash key for write
    pub const FXKEY: u16 = 0x3300;  // for use with XOR instruction

    pub const ERASE: u16  = 0x0002;  // Enable bit for Flash segment erase
    pub const MERAS: u16  = 0x0004;  // Enable bit for Flash mass erase
    pub const WRT: u16    = 0x0040;  // Enable bit for Flash write
    pub const BLKWRT: u16 = 0x0080;  // Enable bit for Flash segment write
    pub const SEGWRT: u16 = 0x0080;  // old definition */ /* Enable bit for Flash segment write

    pub const FN0: u16 = 0x0001;  // Divide Flash clock by 1 to 64 using FN0 to FN5 according to:
    pub const FN1: u16 = 0x0002;  // 32*FN5 + 16*FN4 + 8*FN3 + 4*FN2 + 2*FN1 + FN0 + 1
    //    #ifndef FN2
    pub const FN2: u16 = 0x0004;
    //    #endif
    //    #ifndef FN3
    pub const FN3: u16 = 0x0008;
    //    #endif
    //    #ifndef FN4
    pub const FN4: u16 = 0x0010;
    //    #endif
    pub const FN5: u16 = 0x0020;
    pub const FSSEL0: u16 = 0x0040;  // Flash clock select 0 */        /* to distinguish from USART SSELx
    pub const FSSEL1: u16 = 0x0080;  // Flash clock select 1

    pub const FSSEL_0: u16 = 0x0000;  // Flash clock select: 0 - ACLK
    pub const FSSEL_1: u16 = 0x0040;  // Flash clock select: 1 - MCLK
    pub const FSSEL_2: u16 = 0x0080;  // Flash clock select: 2 - SMCLK
    pub const FSSEL_3: u16 = 0x00C0;  // Flash clock select: 3 - SMCLK

    pub const BUSY: u16 = 0x0001;  // Flash busy: 1
    pub const KEYV: u16 = 0x0002;  // Flash Key violation flag
    pub const ACCVIFG: u16 = 0x0004;  // Flash Access violation flag
    pub const WAIT: u16 = 0x0008;  // Wait flag for segment write
    pub const LOCK: u16 = 0x0010;  // Lock bit: 1 - Flash is locked (read only)
    pub const EMEX: u16 = 0x0020;  // Flash Emergency Exit
    pub const LOCKA: u16 = 0x0040;  // Segment A Lock bit: read = 1 - Segment is locked (read only)
    pub const FAIL: u16 = 0x0080;  // Last Program or Erase failed

    /************************************************************
     * DIGITAL I/O Port1/2 Pull up / Pull down Resistors
     ************************************************************/
    pub const __MSP430_HAS_PORT1_R__: bool = true; // Definition to show that Module is available
    pub const __MSP430_HAS_PORT2_R__: bool = true; // Definition to show that Module is available

    pub const P1IN_: u16 = 0x0020;    // Port 1 Input
    //        const_sfrb(P1IN, P1IN_);
    pub const P1OUT_: u16 = 0x0021;    // Port 1 Output
    //sfrb(P1OUT, P1OUT_);
    pub const P1DIR_: u16 = 0x0022;    // Port 1 Direction
    //sfrb(P1DIR, P1DIR_);
    pub const P1IFG_: u16 = 0x0023;    // Port 1 Interrupt Flag
    //sfrb(P1IFG, P1IFG_);
    pub const P1IES_: u16 = 0x0024;    // Port 1 Interrupt Edge Select
    //sfrb(P1IES, P1IES_);
    pub const P1IE_: u16 = 0x0025;    // Port 1 Interrupt Enable
    //sfrb(P1IE, P1IE_);
    pub const P1SEL_: u16 = 0x0026;    // Port 1 Selection
    //sfrb(P1SEL, P1SEL_);
    pub const P1REN_: u16 = 0x0027;    // Port 1 Resistor Enable
    //sfrb(P1REN, P1REN_);

    pub const P2IN_: u16 = 0x0028;    // Port 2 Input
    //const_sfrb(P2IN, P2IN_);
    pub const P2OUT_: u16 = 0x0029;    // Port 2 Output
    //sfrb(P2OUT, P2OUT_);
    pub const P2DIR_: u16 = 0x002A;    // Port 2 Direction
    //sfrb(P2DIR, P2DIR_);
    pub const P2IFG_: u16 = 0x002B;    // Port 2 Interrupt Flag
    //sfrb(P2IFG, P2IFG_);
    pub const P2IES_: u16 = 0x002C;    // Port 2 Interrupt Edge Select
    //sfrb(P2IES, P2IES_);
    pub const P2IE_: u16 = 0x002D;    // Port 2 Interrupt Enable
    //sfrb(P2IE, P2IE_);
    pub const P2SEL_: u16 = 0x002E;    // Port 2 Selection
    //sfrb(P2SEL, P2SEL_);
    pub const P2REN_: u16 = 0x002F;    // Port 2 Resistor Enable
    //sfrb(P2REN, P2REN_);

    // Timer A2
    pub const __MSP430_HAS_TA2__: bool = true; // Definition to show that Module is available
    pub const TAIV_: u16 = 0x012E;    // Timer A Interrupt Vector Word
    pub const TAIV: u16 = TAIV_;    // Timer A Interrupt Vector Word
    pub const TACTL_: u16 = 0x0160;    // Timer A Control
    pub const TACTL: u16 = TACTL_;    // Timer A Control
    pub const TACCTL0_: u16 = 0x0162;    // Timer A Capture/Compare Control 0
    pub const TACCTL0: u16 = TACCTL0_;    // Timer A Capture/Compare Control 0
    pub const TACCTL1_: u16 = 0x0164;    // Timer A Capture/Compare Control 1
    pub const TACCTL1: u16 = TACCTL1_;    // Timer A Capture/Compare Control 1
    pub const TAR_: u16 = 0x0170;    // Timer A Counter Register
    pub const TAR: u16 = TAR_;    // Timer A Counter Register
    pub const TACCR0_: u16 = 0x0172;    // Timer A Capture/Compare 0
    pub const TACCR0: u16 = TACCR0_;    // Timer A Capture/Compare 0
    pub const TACCR1_: u16 = 0x0174;    // Timer A Capture/Compare 1
    pub const TACCR1: u16 = TACCR1_;    // Timer A Capture/Compare 1

    // Alternate register names
    pub const CCTL0: u16 = TACCTL0;   // Timer A Capture/Compare Control 0
    pub const CCTL1: u16 = TACCTL1;   // Timer A Capture/Compare Control 1
    pub const CCR0: u16 = TACCR0;    // Timer A Capture/Compare 0
    pub const CCR1: u16 = TACCR1;    // Timer A Capture/Compare 1
    pub const CCTL0_: u16 = TACCTL0_;  // Timer A Capture/Compare Control 0
    pub const CCTL1_: u16 = TACCTL1_;  // Timer A Capture/Compare Control 1
    pub const CCR0_: u16 = TACCR0_;   // Timer A Capture/Compare 0
    pub const CCR1_: u16 = TACCR1_;   // Timer A Capture/Compare 1
    // Alternate register names - 5xx style
    pub const TA0IV: u16 = TAIV;      // Timer A Interrupt Vector Word
    pub const TA0CTL: u16 = TACTL;     // Timer A Control
    pub const TA0CCTL0: u16 = TACCTL0;   // Timer A Capture/Compare Control 0
    pub const TA0CCTL1: u16 = TACCTL1;   // Timer A Capture/Compare Control 1
    pub const TA0R: u16 = TAR;       // Timer A Counter Register
    pub const TA0CCR0: u16 = TACCR0;    // Timer A Capture/Compare 0
    pub const TA0CCR1: u16 = TACCR1;    // Timer A Capture/Compare 1
    pub const TA0IV_: u16 = TAIV_;     // Timer A Interrupt Vector Word
    pub const TA0CTL_: u16 = TACTL_;    // Timer A Control
    pub const TA0CCTL0_: u16 = TACCTL0_;  // Timer A Capture/Compare Control 0
    pub const TA0CCTL1_: u16 = TACCTL1_;  // Timer A Capture/Compare Control 1
    pub const TA0R_: u16 = TAR_;      // Timer A Counter Register
    pub const TA0CCR0_: u16 = TACCR0_;   // Timer A Capture/Compare 0
    pub const TA0CCR1_: u16 = TACCR1_;   // Timer A Capture/Compare 1
    pub const TIMER0_A1_VECTOR: u16 = TIMERA1_VECTOR; // Int. Vector: Timer A CC1-2, TA
    pub const TIMER0_A0_VECTOR: u16 = TIMERA0_VECTOR; // Int. Vector: Timer A CC0

    pub const TASSEL1: u16 = 0x0200;  // Timer A clock source select 1
    pub const TASSEL0: u16 = 0x0100;  // Timer A clock source select 0
    pub const ID1: u16 = 0x0080;  // Timer A clock input divider 1
    pub const ID0: u16 = 0x0040;  // Timer A clock input divider 0
    pub const MC1: u16 = 0x0020;  // Timer A mode control 1
    pub const MC0: u16 = 0x0010;  // Timer A mode control 0
    pub const TACLR: u16 = 0x0004;  // Timer A counter clear
    pub const TAIE: u16 = 0x0002;  // Timer A counter interrupt enable
    pub const TAIFG: u16 = 0x0001;  // Timer A counter interrupt flag

    pub const MC_0: u16 = 0x0000;  // Timer A mode control: 0 - Stop
    pub const MC_1: u16 = 0x0010;  // Timer A mode control: 1 - Up to CCR0
    pub const MC_2: u16 = 0x0020;  // Timer A mode control: 2 - Continous up
    pub const MC_3: u16 = 0x0030;  // Timer A mode control: 3 - Up/Down
    pub const ID_0: u16 = 0x0000;  // Timer A input divider: 0 - /1
    pub const ID_1: u16 = 0x0040;  // Timer A input divider: 1 - /2
    pub const ID_2: u16 = 0x0080;  // Timer A input divider: 2 - /4
    pub const ID_3: u16 = 0x00C0;  // Timer A input divider: 3 - /8
    pub const TASSEL_0: u16 = 0x0000; // Timer A clock source select: 0 - TACLK
    pub const TASSEL_1: u16 = 0x0100; // Timer A clock source select: 1 - ACLK
    pub const TASSEL_2: u16 = 0x0200; // Timer A clock source select: 2 - SMCLK
    pub const TASSEL_3: u16 = 0x0300; // Timer A clock source select: 3 - INCLK

    pub const CM1: u16 = 0x8000;  // Capture mode 1
    pub const CM0: u16 = 0x4000;  // Capture mode 0
    pub const CCIS1: u16 = 0x2000;  // Capture input select 1
    pub const CCIS0: u16 = 0x1000;  // Capture input select 0
    pub const SCS: u16 = 0x0800;  // Capture sychronize
    pub const SCCI: u16 = 0x0400;  // Latched capture signal (read)
    pub const CAP: u16 = 0x0100;  // Capture mode: 1 /Compare mode : 0
    pub const OUTMOD2: u16 = 0x0080;  // Output mode 2
    pub const OUTMOD1: u16 = 0x0040;  // Output mode 1
    pub const OUTMOD0: u16 = 0x0020;  // Output mode 0
    pub const CCIE: u16 = 0x0010;  // Capture/compare interrupt enable
    pub const CCI: u16 = 0x0008;  // Capture input signal (read)
    pub const OUT: u16 = 0x0004;  // PWM Output signal if output mode 0
    pub const COV: u16 = 0x0002;  // Capture/compare overflow flag
    pub const CCIFG: u16 = 0x0001;  // Capture/compare interrupt flag

    pub const OUTMOD_0: u16 = 0x0000;  // PWM output mode: 0 - output only
    pub const OUTMOD_1: u16 = 0x0020;  // PWM output mode: 1 - set
    pub const OUTMOD_2: u16 = 0x0040;  // PWM output mode: 2 - PWM toggle/reset
    pub const OUTMOD_3: u16 = 0x0060;  // PWM output mode: 3 - PWM set/reset
    pub const OUTMOD_4: u16 = 0x0080;  // PWM output mode: 4 - toggle
    pub const OUTMOD_5: u16 = 0x00A0;  // PWM output mode: 5 - Reset
    pub const OUTMOD_6: u16 = 0x00C0;  // PWM output mode: 6 - PWM toggle/set
    pub const OUTMOD_7: u16 = 0x00E0;  // PWM output mode: 7 - PWM reset/set
    pub const CCIS_0: u16 = 0x0000; // Capture input select: 0 - CCIxA
    pub const CCIS_1: u16 = 0x1000; // Capture input select: 1 - CCIxB
    pub const CCIS_2: u16 = 0x2000; // Capture input select: 2 - GND
    pub const CCIS_3: u16 = 0x3000; // Capture input select: 3 - Vcc
    pub const CM_0: u16 = 0x0000; // Capture mode: 0 - disabled
    pub const CM_1: u16 = 0x4000; // Capture mode: 1 - pos. edge
    pub const CM_2: u16 = 0x8000; // Capture mode: 1 - neg. edge
    pub const CM_3: u16 = 0xC000; // Capture mode: 1 - both edges

    // TA2IV Definitions
    pub const TAIV_NONE: u16 = 0x0000;    // No Interrupt pending
    pub const TAIV_TACCR1: u16 = 0x0002;    // TACCR1_CCIFG
    pub const TAIV_2: u16 = 0x0004;    // Reserved
    pub const TAIV_6: u16 = 0x0006;    // Reserved
    pub const TAIV_8: u16 = 0x0008;    // Reserved
    pub const TAIV_TAIFG: u16 = 0x000A;    // TAIFG

    // Alternate register names - 5xx style
    pub const TA0IV_NONE: u16 = 0x0000;    // No Interrupt pending
    pub const TA0IV_TA0CCR1: u16 = 0x0002;    // TA0CCR1_CCIFG
    pub const TA0IV_2: u16 = 0x0004;    // Reserved
    pub const TA0IV_6: u16 = 0x0006;    // Reserved
    pub const TA0IV_8: u16 = 0x0008;    // Reserved
    pub const TA0IV_TA0IFG: u16 = 0x000E;    // TA0IFG

    // WATCHDOG TIMER
    pub const __MSP430_HAS_WDT__: bool = true;
    pub const WDTCTL_: *const u16 = 0x0120 as *mut u16;
    pub const WDTCTL: *const u16 = WDTCTL_;
    pub const WDTIS0: u16    = 0x0001;
    pub const WDTIS1: u16    = 0x0002;
    pub const WDTSSEL: u16   = 0x0004;
    pub const WDTCNTCL: u16  = 0x0008;
    pub const WDTTMSEL: u16  = 0x0010;
    pub const WDTNMI: u16    = 0x0020;
    pub const WDTNMIES: u16  = 0x0040;
    pub const WDTHOLD: u16   = 0x0080;
    pub const WDTPW: u16     = 0x5A00;

    pub const __MSP430G2211: bool = true;
    pub const __MSP430_TI_HEADERS__: bool = true;
    // WDT-interval times [1ms] coded with Bits 0-2
    // WDT is clocked by fSMCLK (assumed 1MHz)
    pub const WDT_MDLY_32: u16 = WDTPW+WDTTMSEL+WDTCNTCL;                         // 32ms interval (default)
    pub const WDT_MDLY_8: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTIS0;                  // 8ms     "
    pub const WDT_MDLY_0_5: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTIS1;                  // 0.5ms   "
    pub const WDT_MDLY_0_064: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTIS1+WDTIS0;           // 0.064ms "
    // WDT is clocked by fACLK (assumed 32KHz)
    pub const WDT_ADLY_1000: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTSSEL;                 // 1000ms  "
    pub const WDT_ADLY_250: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTSSEL+WDTIS0;          // 250ms   "
    pub const WDT_ADLY_16: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTSSEL+WDTIS1;          // 16ms    "
    pub const WDT_ADLY_1_9: u16 = WDTPW+WDTTMSEL+WDTCNTCL+WDTSSEL+WDTIS1+WDTIS0;   // 1.9ms   "
    // Watchdog mode -> reset after expired time
    // WDT is clocked by fSMCLK (assumed 1MHz)
    pub const WDT_MRST_32: u16 = WDTPW+WDTCNTCL;                                  // 32ms interval (default)
    pub const WDT_MRST_8: u16 = WDTPW+WDTCNTCL+WDTIS0;                           // 8ms     "
    pub const WDT_MRST_0_5: u16 = WDTPW+WDTCNTCL+WDTIS1;                           // 0.5ms   "
    pub const WDT_MRST_0_064: u16 = WDTPW+WDTCNTCL+WDTIS1+WDTIS0;                    // 0.064ms "
    // WDT is clocked by fACLK (assumed 32KHz)
    pub const WDT_ARST_1000: u16 = WDTPW+WDTCNTCL+WDTSSEL;                          // 1000ms  "
    pub const WDT_ARST_250: u16 = WDTPW+WDTCNTCL+WDTSSEL+WDTIS0;                   // 250ms   "
    pub const WDT_ARST_16: u16 = WDTPW+WDTCNTCL+WDTSSEL+WDTIS1;                   // 16ms    "
    pub const WDT_ARST_1_9: u16 = WDTPW+WDTCNTCL+WDTSSEL+WDTIS1+WDTIS0;            // 1.9ms   "

    // INTERRUPT CONTROL
    // These two bits are defined in the Special Function Registers
    // pub const WDTIE: u16 = 0x01
    // pub const WDTIFG: u16 = 0x01

    /************************************************************
     * Calibration Data in Info Mem
     ************************************************************/

    //    #ifndef __DisableCalData

    pub const CALDCO_1MHZ_: u16 = 0x10FE;    // DCOCTL  Calibration Data for 1MHz
    //const_sfrb(CALDCO_1MHZ, CALDCO_1MHZ_);
    pub const CALBC1_1MHZ_: u16 = 0x10FF;    // BCSCTL1 Calibration Data for 1MHz
    //const_sfrb(CALBC1_1MHZ, CALBC1_1MHZ_);

    //    #endif // #ifndef __DisableCalData

    /************************************************************
     * Interrupt Vectors (offset from 0xFFE0)
     ************************************************************/

    pub const PORT1_VECTOR: u16 = 0x0004;  // 0xFFE4 Port 1
    pub const PORT2_VECTOR: u16 = 0x0006;  // 0xFFE6 Port 2
    pub const TIMERA1_VECTOR: u16 = 0x0010;  // 0xFFF0 Timer A CC1-2, TA
    pub const TIMERA0_VECTOR: u16 = 0x0012;  // 0xFFF2 Timer A CC0
    pub const WDT_VECTOR: u16 = 0x0014; // 0xFFF4 Watchdog Timer
    pub const COMPARATORA_VECTOR: u16 = 0x0016; // 0xFFF6 Comparator A
    pub const NMI_VECTOR: u16 = 0x001C; // 0xFFFC Non-maskable
    pub const RESET_VECTOR: u16 = 0x001E; // 0xFFFE Reset [Highest Priority]
}
