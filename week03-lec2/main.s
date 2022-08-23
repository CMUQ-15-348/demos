; Put my field constants
SYSCTL_RCGC2_R			.field	0x400FE108,32
GPIO_PORTF_DATA_BITS_R	.field	0x40025000,32
GPIO_PORTF_DATA_R		.field	0x400253FC,32
GPIO_PORTF_LOCK_R		.field	0x40025520,32
GPIO_PORTF_CR_R			.field	0x40025524,32
GPIO_PORTF_AMSEL_R		.field	0x40025528,32
GPIO_PORTF_PCTL_R		.field	0x4002552C,32
GPIO_PORTF_DIR_R		.field	0x40025400,32
GPIO_PORTF_AFSEL_R		.field	0x40025420,32
GPIO_PORTF_DEN_R		.field	0x4002551C,32
GPIO_PORTF_PUR_R		.field	0x40025510,32

STUPID_LOCK_VALUE	.field 0x4C4F434B,32

	.text

init_portf:
    ;SYSCTL_RCGC2_R |= 0x00000020;     // 1) F clock
    LDR R0, SYSCTL_RCGC2_R
    LDR R1, [R0]
    ORR R1, R1, #0x20
	STR R1, [R0]

    ;GPIO_PORTF_LOCK_R = 0x4C4F434B;   // 2) unlock PortF PF0
    LDR R0, GPIO_PORTF_LOCK_R
    LDR R1, STUPID_LOCK_VALUE
    STR R1, [R0]

    ;GPIO_PORTF_CR_R = 0x1F;           // 3) allow changes to PF4-0
    LDR R0, GPIO_PORTF_CR_R
	MOV R1, #0x1f
	STR R1, [R0]

  	;GPIO_PORTF_AMSEL_R = 0x00;        	// 4) disable analog function
  	LDR R0, GPIO_PORTF_AMSEL_R
  	MOV R1, #0x00
  	STR R1, [R0]

  	;GPIO_PORTF_AFSEL_R = 0x00;        	// 5) no alternate function
  	LDR R0, GPIO_PORTF_AFSEL_R
  	MOV R1, #0x00
  	STR R1, [R0]

  	;GPIO_PORTF_PCTL_R = 0x00000000;   	// 6) GPIO clear bit PCTL
  	LDR R0, GPIO_PORTF_PCTL_R
  	MOV R1, #0x00
  	STR R1, [R0]

  	;GPIO_PORTF_PUR_R = 0x11;          	// 7) enable pullup resistors on PF4,PF0
  	LDR R0, GPIO_PORTF_PUR_R
  	MOV R1, #0x11
  	STR R1, [R0]

  	;GPIO_PORTF_DEN_R = 0x1F;          	// 8) enable digital pins PF4-PF0
  	LDR R0, GPIO_PORTF_DEN_R
  	MOV R1, #0x1f
  	STR R1, [R0]

  	;GPIO_PORTF_DIR_R = 0x0E;          	// 9) PF4,PF0 input, PF3,PF2,PF1 output
  	LDR R0, GPIO_PORTF_DIR_R
  	MOV R1, #0x0E
  	STR R1, [R0]

  	BX LR

	.global main
main:
	BL	init_portf

	; I want the address of the Port F Data Register available
	LDR R5, GPIO_PORTF_DATA_R

	; Make the light green initially
	MOV R1, #0x08 ; green
	STR R1, [R5]

main_loop:
	; Read from Port F
	LDR R6,[R5]

	; Check if SW1 is pressed (0x10)
	ANDS R7, R6, #0x10
	BNE skip_sw1
	; If I get here, then the switch was pressed
	MOV R1, #0x02 ; red
	STR R1, [R5]
skip_sw1:
	; Check if SW2 is pressed (0x01)
	ANDS R7, R6, #0x01
	BNE skip_sw2
	; If I get here, then the switch was pressed
	MOV R1, #0x04 ; blue
	STR R1, [R5]
skip_sw2:

	B main_loop


	bx LR
