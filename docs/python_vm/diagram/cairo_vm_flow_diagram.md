::: mermaid
graph LR
    main --> ARGS[Add and parse args]
    main --> get_crypto_lib_context_managers
    main --> cairo_run
    cairo_run --> load_program
 
    cairo_run --> initialize_segments
    initialize_segments --> add
    initialize_segments --> ini_seg[initialize_segments]
    ini_seg --> add2[add]

    cairo_run --> initialize_main_entrypoint
    initialize_main_entrypoint --> initial_stack
    initialize_main_entrypoint --> add3[add]
    initialize_main_entrypoint --> initialize_function_entrypoint
    initialize_function_entrypoint --> add4[add]
    initialize_function_entrypoint --> initialize_state
    initialize_state --> load_data 
    load_data --> load_data_seg[load_data]
    
    cairo_run --> initialize_vm
    initialize_vm --> add_validation_rules
    initialize_vm --> add_auto_deduction_rules
    initialize_vm --> validate_existing_memory
    validate_existing_memory --> val_ex_mem[validate_existing_memory]
    val_ex_mem --> validate_memory_cell
    
    cairo_run --> run_until_pc
    run_until_pc --> vm_step
    vm_step --> step
    vm_step --> consume_step
    step --> hint[Hint Management]
    step --> decode_current_instruction
    decode_current_instruction --> get_instruction_encoding
    get_instruction_encoding --> get
    decode_current_instruction --> decode_instruction
    decode_instruction --> decode_instruction_values
    step --> run_instruction
    run_instruction --> compute_operands
    compute_operands --> compute_dst_address
    compute_operands --> compute_op0_address
    compute_operands --> compute_op1_address
    compute_operands --> deduce_memory_cell
    compute_operands --> deduce_op0
    compute_operands --> deduce_op1
    compute_operands --> deduce_res
    run_instruction --> opcode_assertions
    run_instruction --> update_registers

    cairo_run --> end_run
    end_run --> rel_val[relocate_value]
    end_run --> relocate_memory
    relocate_memory --> rel_val
    end_run --> end_run_vm[end_run]
    end_run_vm --> verify_auto_deductions
    end_run --> freeze
    end_run --> compute_effective_sizes


    cairo_run --> rel[relocate]
    rel --> relocate_segments
    relocate_segments --> get_segment_size
    get_segment_size --> get_sus[get_segment_used_size]
    rel --> rel_val_runner[relocate_value]
    rel_val_runner --> rel_val_rel[relocate_value]
    rel --> relocate_trace
    rel --> relocate_builtin[relocate]
    

    cairo_run --> print_output
    print_output --> get_used_cells_and_allocated_size
    get_used_cells_and_allocated_size --> get_used_cells
    get_used_cells --> get_segment_used_size
    print_output --> get2[get]

    style main fill:#CBE3CA,color:0
    style cairo_run fill:#CBE3CA,color:0
    style load_program fill:#CBE3CA,color:0
    style initialize_segments fill:#F5C1C1,color:0
    style initialize_state fill:#F5C1C1,color:0
    style initialize_main_entrypoint fill:#F5C1C1,color:0
    style initialize_vm fill:#F5C1C1,color:0
    style run_until_pc fill:#F5C1C1,color:0
    style end_run fill:#F5C1C1,color:0
    style print_output fill:#F5C1C1,color:0
    style rel fill:#F5C1C1,color:0
    style rel_val_runner fill:#F5C1C1,color:0
    style vm_step fill:#F5C1C1,color:0
    style validate_existing_memory fill:#FFE0C1,color:0
    style ini_seg fill:#6495ed,color:0
    style add fill:#20b2aa,color:0
    style add2 fill:#20b2aa,color:0
    style add3 fill:#20b2aa,color:0
    style add4 fill:#20b2aa,color:0
    style initial_stack fill:#6495ed,color:0
    style initialize_function_entrypoint fill:#F5C1C1,color:0
    style load_data fill:#F5C1C1,color:0
    style load_data_seg fill:#20b2aa,color:0
    style add_validation_rules fill:#6495ed,color:0
    style add_auto_deduction_rules fill:#6495ed,color:0
    style val_ex_mem fill:#ffb3de,color:0
    style validate_memory_cell fill:#ffb3de,color:0
    style step fill:#ffcc99,color:0
    style hint fill:#ffcc99,color:0
    style decode_current_instruction fill:#ffcc99,color:0
    style compute_operands fill:#ffcc99,color:0
    style opcode_assertions fill:#ffcc99,color:0
    style get_instruction_encoding fill:#ffcc99,color:0
    style compute_op1_address fill:#ffcc99,color:0
    style compute_op0_address fill:#ffcc99,color:0
    style compute_dst_address fill:#ffcc99,color:0
    style deduce_op0 fill:#ffcc99,color:0
    style deduce_op1 fill:#ffcc99,color:0
    style deduce_res fill:#ffcc99,color:0
    style update_registers fill:#ffcc99,color:0
    style deduce_memory_cell fill:#FFE0C1,color:0
    style run_instruction fill:#ffcc99,color:0
    style decode_instruction_values fill:#fdfd96,color:0
    style decode_instruction fill: #ccccff,color:0
    style get fill: #98ff98,color:0
    style get2 fill: #98ff98,color:0
    style consume_step fill:#f5f5f5,color:0
    style rel_val fill: #98ff98,color:0
    style relocate_memory fill: #98ff98,color:0
    style end_run_vm fill:#FFE0C1,color:0
    style verify_auto_deductions fill:#FFE0C1,color:0
    style freeze fill: #98ff98,color:0
    style compute_effective_sizes fill:#20b2aa,color:0
    style get_used_cells_and_allocated_size fill:#acbf60,color:0
    style get_used_cells fill:#acbf60,color:0
    style get_segment_used_size fill:#20b2aa,color:0
    style relocate_segments fill:#20b2aa,color:0
    style get_segment_size fill:#20b2aa,color:0
    style get_sus fill:#20b2aa,color:0
    style rel_val_rel fill:#7df9ff,color:0
    style relocate_trace fill:#ba8759,color:0
    style relocate_builtin fill:#6495ed,color:0
:::
