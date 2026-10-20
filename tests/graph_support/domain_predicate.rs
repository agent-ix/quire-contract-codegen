// Independent original-domain membership over verification storage, never production arrays.
#[derive(Clone, Debug)]
struct CandidateGraph {
    count: i64,
    start: i64,
    target: i64,
    degrees: [i64; 5],
    edges: [[i64; 4]; 5],
}

#[derive(Clone, Copy, Debug)]
struct AuthoredDomain {
    maximum: i64,
    lower: i64,
    upper: i64,
    unique: bool,
    same_operand: bool,
}

fn authored_admits(g: &CandidateGraph, d: AuthoredDomain) -> bool {
    if g.count < 0 || g.count > d.maximum {
        return false;
    }
    if g.count == 0 {
        return true;
    }
    if g.start < 0 || g.start >= g.count || g.target < 0 || g.target >= g.count
        || (d.same_operand && g.start != g.target)
    {
        return false;
    }
    for row in 0..5 {
        if row as i64 >= g.count {
            continue;
        }
        let degree = g.degrees[row];
        if degree < d.lower || degree > d.upper {
            return false;
        }
        for slot in 0..4 {
            if slot as i64 >= degree {
                continue;
            }
            let target = g.edges[row][slot];
            if target < 0 || target >= g.count {
                return false;
            }
            if d.unique {
                for previous in 0..slot {
                    if target == g.edges[row][previous] {
                        return false;
                    }
                }
            }
        }
    }
    true
}
