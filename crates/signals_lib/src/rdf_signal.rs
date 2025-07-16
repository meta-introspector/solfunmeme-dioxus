use sophia_term::BoxTerm;
use sophia::parser::turtle;
use sophia::triple::stream::TripleSource;
use crate::common::{SignalManager, SignalState};

pub type RdfGraph = Vec<[BoxTerm; 3]>;
pub type RdfSignal = SignalManager<RdfGraph>;

/// Create a new RDF signal from Turtle data
pub fn new_rdf_signal_from_turtle(turtle_data: &str) -> RdfSignal {
    let mut graph: RdfGraph = Vec::new();
    let triples: Vec<[BoxTerm; 3]> = turtle::parse_str(turtle_data)
        .collect_triples()
        .expect("Failed to parse Turtle");
    graph.extend(triples);
    SignalManager::new(graph)
}

/// Add triples to the RDF signal from Turtle data
pub fn add_triples_from_turtle(signal: &mut RdfSignal, turtle_data: &str) {
    let mut graph = signal.get();
    let triples: Vec<[BoxTerm; 3]> = turtle::parse_str(turtle_data)
        .collect_triples()
        .expect("Failed to parse Turtle");
    graph.extend(triples);
    signal.set_success(graph);
}

/// Query the RDF signal for triples matching a pattern
pub fn query_triples(signal: &RdfSignal, subj: Option<&str>, pred: Option<&str>, obj: Option<&str>) -> Vec<(String, String, String)> {
    let graph = signal.get();
    let mut results = Vec::new();
    for t in &graph {
        let s = t[0].to_string();
        let p = t[1].to_string();
        let o = t[2].to_string();
        if subj.map_or(true, |s_| s_ == s)
            && pred.map_or(true, |p_| p_ == p)
            && obj.map_or(true, |o_| o_ == o)
        {
            results.push((s, p, o));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rdf_signal() {
        let turtle = "@prefix ex: <http://example.org/> . ex:a ex:b ex:c .";
        let mut signal = new_rdf_signal_from_turtle(turtle);
        let triples = query_triples(&signal, None, None, None);
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].0, "http://example.org/a");
        assert_eq!(triples[0].1, "http://example.org/b");
        assert_eq!(triples[0].2, "http://example.org/c");
        // Add another triple
        add_triples_from_turtle(&mut signal, "ex:x ex:y ex:z .");
        let triples = query_triples(&signal, None, None, None);
        assert_eq!(triples.len(), 2);
    }
} 