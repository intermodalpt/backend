/*
    Intermodal, transportation information aggregator
    Copyright (C) 2022 - 2023  Cláudio Pereira

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

pub fn needleman_wunsch<'a, T: PartialEq + Clone>(
    seq1: &'a [T],
    seq2: &'a [T],
) -> (Vec<Option<&'a T>>, Vec<Option<&'a T>>, Vec<Vec<i32>>) {
    let n = seq1.len();
    let m = seq2.len();

    let gap_penalty = -1;
    let match_score = 1;
    let mismatch_score = -1;

    let mut matrix = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        matrix[i][0] = i as i32 * gap_penalty;
    }

    for j in 0..=m {
        matrix[0][j] = j as i32 * gap_penalty;
    }

    for i in 1..=n {
        for j in 1..=m {
            let match_ = matrix[i - 1][j - 1]
                + if seq1[i - 1] == seq2[j - 1] {
                    match_score
                } else {
                    mismatch_score
                };
            let delete_score = matrix[i - 1][j] + gap_penalty;
            let insert_score = matrix[i][j - 1] + gap_penalty;
            matrix[i][j] =
                match [match_, delete_score, insert_score].iter().max() {
                    Some(x) => *x,
                    None => 0,
                };
        }
    }

    let mut aligned_seq1 = vec![];
    let mut aligned_seq2 = vec![];

    let mut i = n;
    let mut j = m;

    while i > 0 || j > 0 {
        if i > 0
            && j > 0
            && matrix[i][j]
                == matrix[i - 1][j - 1]
                    + if seq1[i - 1] == seq2[j - 1] {
                        match_score
                    } else {
                        mismatch_score
                    }
        {
            aligned_seq1.push(Some(&seq1[i - 1]));
            aligned_seq2.push(Some(&seq2[j - 1]));
            i -= 1;
            j -= 1;
        } else if i > 0 && matrix[i][j] == matrix[i - 1][j] + gap_penalty {
            // Up, space added to seq2
            aligned_seq1.push(Some(&seq1[i - 1]));
            aligned_seq2.push(None);
            i -= 1;
        } else {
            // Left, space added to seq1
            aligned_seq1.push(None);
            aligned_seq2.push(Some(&seq2[j - 1]));
            j -= 1;
        }
    }

    aligned_seq1.reverse();
    aligned_seq2.reverse();

    (aligned_seq1, aligned_seq2, matrix)
}

pub(crate) fn stop_seq_error<T: PartialEq + Clone>(
    seq1: &[T],
    seq2: &[T],
) -> (usize, usize) {
    let (aligned1, aligned2, _) = needleman_wunsch(seq1, seq2);
    let mut matches = 0;
    let mut mismatches = 0;
    for (e1, e2) in aligned1.iter().zip(aligned2.iter()) {
        if e1 == e2 {
            matches += 1;
        } else {
            mismatches += 1;
        }
    }
    (matches, mismatches)
}
