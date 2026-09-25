//! Modul sector_loader dla skrzynki m2-client-terrain
//! Menedzer asynchronicznego ladowania sektorow mapy 3x3
//! Streaming sektorow terenu wokol pozycji gracza, doczytywanie w tle

use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Identyfikator sektora oparty na wspolrzednych X i Y na mapie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorId {
    pub x: i32,
    pub y: i32,
}

/// Reprezentuje dane zaladowanego sektora
#[derive(Debug, Clone)]
pub struct SectorData {
    pub id: SectorId,
    pub is_loaded: bool,
    // W pelnej implementacji znajdowalyby sie tutaj np. mapy wysokosci, obiekty 3D itp.
}

/// Zlecenie zaladowania lub wyladowania sektora do watku pracownika
enum WorkerCommand {
    Load(SectorId),
    Terminate,
}

/// Glowna struktura implementacyjna ladowarki sektorow
#[derive(Debug)]
pub struct SectorLoader {
    initialized: bool,
    loaded_sectors: HashMap<SectorId, SectorData>,
    loading_sectors: HashSet<SectorId>,
    command_sender: Sender<WorkerCommand>,
    data_receiver: Receiver<SectorData>,
    worker_handle: Option<JoinHandle<()>>,
}

impl Default for SectorLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl SectorLoader {
    /// Tworzy nowa instancje menedzera i uruchamia watek pracownika
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (data_tx, data_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            Self::worker_thread(cmd_rx, data_tx);
        });

        Self {
            initialized: true,
            loaded_sectors: HashMap::new(),
            loading_sectors: HashSet::new(),
            command_sender: cmd_tx,
            data_receiver: data_rx,
            worker_handle: Some(handle),
        }
    }

    /// Funkcja watku pracownika, ktora symuluje ladowanie danych z dysku lub sieci
    fn worker_thread(rx: Receiver<WorkerCommand>, tx: Sender<SectorData>) {
        for cmd in rx {
            match cmd {
                WorkerCommand::Load(id) => {
                    // W prawdziwym srodowisku uzylibysmy asynchronicznego IO lub dluzej trwajacego zadania
                    let data = SectorData {
                        id,
                        is_loaded: true,
                    };
                    // Wyslanie z powrotem ignoruje blad, jesli glowny watek zostal zamkniety
                    let _ = tx.send(data);
                }
                WorkerCommand::Terminate => {
                    break;
                }
            }
        }
    }

    /// Zwraca czy menedzer zostal zainicjowany poprawnie
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Pobiera przetworzone sektory z kanalu (nie blokuje dzialania)
    pub fn process_loaded_sectors(&mut self) {
        while let Ok(data) = self.data_receiver.try_recv() {
            // Ignorujemy sektory, ktore zdazyly zostac wyrejestrowane z ladowania
            if self.loading_sectors.contains(&data.id) {
                self.loading_sectors.remove(&data.id);
                self.loaded_sectors.insert(data.id, data);
            }
        }
    }

    /// Aktualizuje pozycje srodkowa i zarzadza siatka 3x3 wokol niej
    pub fn update_position(&mut self, center_x: i32, center_y: i32) {
        let mut expected_sectors = HashSet::new();

        // Oblicz siatke 3x3
        for dx in -1..=1 {
            for dy in -1..=1 {
                expected_sectors.insert(SectorId {
                    x: center_x + dx,
                    y: center_y + dy,
                });
            }
        }

        // Znajdz sektory do usuniecia
        let to_remove: Vec<SectorId> = self
            .loaded_sectors
            .keys()
            .chain(self.loading_sectors.iter())
            .filter(|id| !expected_sectors.contains(id))
            .cloned()
            .collect();

        // Usun niepotrzebne sektory
        for id in to_remove {
            self.loaded_sectors.remove(&id);
            self.loading_sectors.remove(&id);
        }

        // Zlec zaladowanie brakujacych sektorow
        for id in expected_sectors {
            if !self.loaded_sectors.contains_key(&id) && !self.loading_sectors.contains(&id) {
                self.loading_sectors.insert(id);
                let _ = self.command_sender.send(WorkerCommand::Load(id));
            }
        }
    }

    /// Pobiera referencje do zaladowanych sektorow
    pub fn get_loaded_sectors(&self) -> &HashMap<SectorId, SectorData> {
        &self.loaded_sectors
    }
}

impl Drop for SectorLoader {
    fn drop(&mut self) {
        let _ = self.command_sender.send(WorkerCommand::Terminate);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}
