pub trait MarkdownParser {
    fn parse(&self, markdown_text: String) -> Option<String>;
}

pub struct NoopMarkdownParser {}

impl MarkdownParser for NoopMarkdownParser {
    fn parse(&self, markdown_text: String) -> Option<String> {
        return Some(markdown_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_markdown_parser_returns_given_string() {
        let parser = NoopMarkdownParser{};
        let input = r#"
            # Anus hic sequerer

            ## Tibi falsaque

            Lorem markdownum **dicere**. Has mater in concipit effice irascentemque tibi
            vestra socium de communem esset, surge manu, sit mora. Timuit **hac saxa
            aspergine** fata aurato, vomit me duri ratos Venerem audebatis Orpheu.

            1. Meo imis operis
            2. Detur Scorpion
            3. Fit stetit imagine intra Quirini et cuius
            4. Iris reminiscor mecum erat currus quae
            5. Cupidine Cereris vallibus Tellus altique agricolis
            6. Ego in

            ## Poenam certa imbres frena queat

            Isdem et medullas audet; quid aequoribus terra tormento, ferox divinante,
            bracchia versata, ut mihi sequuntur. Patientem veniente bracchia, armenta voces
            natumque fluitque vero; casu. Sinus antris, ima illis puerosque altaque oscula
            fumos, ab *de tepido*.

            ## Quoque iamque fontibus

            Deperit lenius in parte linguam, agitantem tenuere pro. **Timidasque armis**,
            nec agnae paenituisse plausus cuncta Cephison in ut est. Nec nubibus signa, nisi
            linigera; in curat tamen designat subito sola, nec moram.

            - Parentes annos et Melampus cuncti
            - Tum atque unam
            - Unda noverat eadem

            ## Tum visa vertigine

            Maxima corymbis me ille ab a parte secabatur faveant veni Quas fracta summam,
            cum erat. Naides si transtra habet in meum dedit, fata quoque Hecabe. Quem
            rapiunt quietis idem, ubi crura spatium venerem, in exemploque minus semperque
            stravit ausam sonumque, est. Et thalamosque coniunx.

            > Arte alia, nec est vultus, fraude vertice parentis quam. Liquerat edocuit
            > inpia pulsata sana choreas mente, de agros vertitur, medicamine, aethera. Et
            > sibi, noverat delius, ligavit, Troica, attulerat sparsi diros. Remisso
            > verborum ferrum, sic periere modo duo videri, tibi. Quos per leonem mihique
            > vera.

            Miraturus et **iaculo** falsus ratem teneros inploraret solamen erat. Quam Ardea
            foedataque vestigia in mensas nigris **nisi** grata!
        "#;
        let result = parser.parse(String::from(input));
        match result {
            Some(_t) => assert!(true),
            None => assert!(false),
        }
    }
}