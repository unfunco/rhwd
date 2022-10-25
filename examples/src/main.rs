rhwd::rhwd! {
    allanol crât rhwd;

    defnyddio std::collections::Geiriadur fel Geir;

    nodwedd GwerthAllwedd {
        ffwythiant i_ysgrifennu(&hunan, gwerth: Llinyn, allwedd: Llinyn);
    }

    #[caniatáu(cod_marw)]
    statig treigladwy GEIRIADUR: Opsiwn<Geir<Llinyn, Llinyn>> = Dim;

    #[caniatáu(treigladwy_heb_ei_ddefnyddio)]
    ffwythiant prif() {
        gadael treigladwy x = 42;
        cyfateb x {
            42 => {
                brintln!("Daffodils!")
            }
            _ => {
                brintln!("Sboncen!")
            }
        }
    }

    #[caniatáu(cod_anghyraeddadwy)]
    ffwythiant eilradd() {
        cachu!("panig!");
        ffyc!("panig!");
    }
}
