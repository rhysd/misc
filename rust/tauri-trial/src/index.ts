import { invoke } from 'tauri/api/tauri';

invoke({ cmd: 'greet', message: 'Hello' });
