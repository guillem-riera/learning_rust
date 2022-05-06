.phony : install_jupyter_tools trust

INSTALL_RUST_NB = 00_Install_Rust.ipynb
INSTALL_RUST_MD = 00_install_rust.md
INTRO_TO_RUST_NB = 01_Intro_to_Rust.ipynb
INTRO_TO_RUST_MD = 01_intro_to_rust.md
INTERESTING_STUFF_NB = 02_Interesting_Stuff.ipynb
INTERESTING_STUFF_MD = 02_interesting_stuff.md
OWNING_AND_BORROWING_NB = 03_Owning_and_Borrowing.ipynb
OWNING_AND_BORROWING_MD = 03_owning_and_borrowing.md

start: trust
	jupyter notebook


trust:
	jupyter trust *.ipynb


${INSTALL_RUST_MD}: ${INSTALL_RUST_NB}
	jupyter nbconvert ${INSTALL_RUST_NB} --to markdown --output ${INSTALL_RUST_MD}

${INTRO_TO_RUST_MD}: ${INTRO_TO_RUST_NB}
	jupyter nbconvert ${INTRO_TO_RUST_NB} --to markdown --output ${INTRO_TO_RUST_MD}

${INTERESTING_STUFF_MD}: ${INTERESTING_STUFF_NB}
	jupyter nbconvert ${INTERESTING_STUFF_NB} --to markdown --output ${INTERESTING_STUFF_MD}

${OWNING_AND_BORROWING_MD}: ${OWNING_AND_BORROWING_NB}
	jupyter nbconvert ${OWNING_AND_BORROWING_NB} --to markdown --output ${OWNING_AND_BORROWING_MD}

INSTALL_RUST_MD: ${INSTALL_RUST_MD}
INTRO_TO_RUST_MD: ${INTRO_TO_RUST_MD}
INTERESTING_STUFF_MD: ${INTERESTING_STUFF_MD}
OWNING_AND_BORROWING: ${OWNING_AND_BORROWING_MD}

MD_DOCS: ${INSTALL_RUST_MD} ${INTERESTING_STUFF_MD} ${INTRO_TO_RUST_MD} ${OWNING_AND_BORROWING_MD}

install_jupyter_tools:
	pip3 install -r jupyter_tools/requirements.txt
