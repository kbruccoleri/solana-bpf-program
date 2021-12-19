### Code organization and flow

Flow of a program:
1. Someone calls the entrypoint
2. Entrypoint forwards arguments to the processor
3. The processor asks `instruction.rs` to decode `instruction_data` from entrypoint function
    - This feels a lot like a "controller".
4. Processor uses decoded data to decide which processing function to use to process the request
5. Processor may use `state.rs` to encode state into or decode state of an account passed to entrypoint