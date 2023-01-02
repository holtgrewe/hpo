use hpo::{Ontology, annotations::{OmimDiseaseId, OmimDisease}, similarity::{GroupSimilarity, StandardCombiner, GraphIc}, term::HpoGroup, HpoTermId, HpoSet};



fn main() {
    let ontology = Ontology::from_binary("tests/ontology.hpo").unwrap();
    let combiner = StandardCombiner::FunSimAvg;
    let similarity = GraphIc::new(hpo::term::InformationContentKind::Omim);
    let sim = GroupSimilarity::new(combiner, similarity);

    let mut args = std::env::args();
    if args.len() == 3 {
        let omimid_a = OmimDiseaseId::try_from(args.nth(1).unwrap().as_str()).expect("The first OmimID is invalid");
        let omimid_b = OmimDiseaseId::try_from(args.next().unwrap().as_str()).expect("The second OmimID is invalid");

        let omim_a = ontology.omim_disease(&omimid_a).expect("The first OMIM Disease is not part of the Ontology");
        let omim_b = ontology.omim_disease(&omimid_b).expect("The second OMIM Disease is not part of the Ontology");

        let set_a = omim_a.to_hpo_set(&ontology);
        let set_b = omim_b.to_hpo_set(&ontology);

        let res = sim.calculate(&set_a, &set_b);
        println!("Similarity is {res}");
    } else if args.len() == 2 {
        let arg = args.nth(1).unwrap();
        let hpo_terms = arg.split(',');
        let mut group = HpoGroup::default();

        let mut all: Vec<(&OmimDisease, f32)> = Vec::new();
        for t in hpo_terms {
            group.insert(HpoTermId::try_from(t).expect("Invalid HpoTermId"));
        }
        let set_a = HpoSet::new(&ontology, group);
        for disease in ontology.omim_diseases() {
            let res = sim.calculate(&set_a, &disease.to_hpo_set(&ontology));
            all.push((disease, res));
        }
        println!("Number of comparisons: {}", all.len());
        all.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for x in all.iter().take(50) {
            println!("{}\t{}\t{}", x.0.id(), x.0.name(), x.1);
        }
    } else if args.len() == 1 {
        let mut all: Vec<(OmimDiseaseId, OmimDiseaseId, f32)> = Vec::new();
        for disease_a in ontology.omim_diseases() {
            for disease_b in ontology.omim_diseases().take(100) {
                let res = sim.calculate(&disease_a.to_hpo_set(&ontology), &disease_b.to_hpo_set(&ontology));
                // println!("{} - {}: {res}", disease_a.id(), disease_b.id());
                all.push((*disease_a.id(), *disease_b.id(), res));
            }
        }
        println!("Number of comparisons: {}", all.len());
    }

}
