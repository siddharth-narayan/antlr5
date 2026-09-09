    // .with_min_len(ir.rules().len().div_ceil(rayon::current_num_threads()))
    // (0..ir.rules().len()).into_par_iter().for_each(
    //     |rule_id| {
    //         let mut cache = HashSetMap::new();
    //         for n in 0..=25 {
    //             let mut set: HashSet<ElementIR, RandomState> = HashSet::default();

    //             for alt in ir.get_rule(rule_id).unwrap().alts() {
    //                 let n = crate::nth(n, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut cache, &mut HashSet::default(), ir.rules()).cloned().unwrap_or_default();
    //                 set.extend(n);
    //             }

    //             // println!("NTH set for n = {}: {:#?}", n, set);
    //             black_box(set);
    //         }

    //         println!("Calculated nth sets for rule {}", rule_id);
    //     }
    // );
