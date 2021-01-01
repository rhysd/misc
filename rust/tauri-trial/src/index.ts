import { invoke } from 'tauri/api/tauri';

invoke({ cmd: 'myCustomCommand', argument: 'Hello' });
