use crate::editor::core::PixelArtEditor;

impl PixelArtEditor {
    pub fn push_undo(&mut self) {
        // Store current state in undo stack
        self.undo_stack.push((self.frames.clone(), self.current_frame, self.current_layer));
        
        // Clear redo stack when making a new change
        self.redo_stack.clear();
        
        // Keep undo stack size reasonable (last 50 states)
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        
        // Also keep the old last_state for compatibility
        self.last_state = Some((self.frames.clone(), self.current_frame, self.current_layer));
    }

    pub fn undo(&mut self) {
        if let Some((frames, cf, cl)) = self.undo_stack.pop() {
            // Push current state to redo stack
            self.redo_stack.push((self.frames.clone(), self.current_frame, self.current_layer));
            
            // Restore previous state
            self.frames = frames;
            self.current_frame = cf;
            self.current_layer = cl;
            
            // Invalidate cache
            self.invalidate_cache();
        }
    }

    pub fn redo(&mut self) {
        if let Some((frames, cf, cl)) = self.redo_stack.pop() {
            // Push current state to undo stack
            self.undo_stack.push((self.frames.clone(), self.current_frame, self.current_layer));
            
            // Restore redo state
            self.frames = frames;
            self.current_frame = cf;
            self.current_layer = cl;
            
            // Invalidate cache
            self.invalidate_cache();
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
