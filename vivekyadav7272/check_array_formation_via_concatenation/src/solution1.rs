pub struct Solution;
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        // *Sigh*, turns out the question guarantees that no piece is empty, nor is the arr empty,
        // so all that philosophising was for nothing.

        // The "problem space", or the part of the arr that has not yet been paired
        // with a piece.
        let mut problem_space = &arr[..];
        
        // Holds the pieces that have not been paired yet.
        let mut unsolved_pieces = Vec::with_capacity(pieces.len());
        for piece in pieces.iter() {
            unsolved_pieces.push(Some(piece));
        }

        // Atleast one element guaranteed.
        loop {
            let first_elem = match problem_space.first() {
                Some(elem) => *elem,
                None => return true,
            };
            let mut no_pieces_match = true;
            let mut to_remove: usize = usize::MAX;
            for (i, unsolved_piece) in unsolved_pieces.iter()
                                                    .enumerate()
                                                    .filter_map(
                                                        |(i,piece)|  
                                                        piece.and_then( |piece_unwrapped| 
                                                            Some((i, piece_unwrapped)))
                                                    ) {
                if first_elem == unsolved_piece[0] {
                    no_pieces_match = false;
                    if problem_space[1..].starts_with(&unsolved_piece[1..]) {
                        problem_space = &problem_space[unsolved_piece.len()..];
                        to_remove = i;
                        break;
                    } else {
                        // Since all i32's in all the pieces are unique, that means
                        // no other piece will have unsolved_piece[0] as their subpiece,
                        // meaning no other pieces can match either.
                        return false;
                    }
                }
            }
            
            if no_pieces_match {
                return false;
            }
            // If control flow reaches here, it must mean that to_remove has already been rewritten to a valid value.

            unsolved_pieces[to_remove] = None;
        }
        unreachable!();
    }
}