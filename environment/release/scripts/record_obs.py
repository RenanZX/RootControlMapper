import obsws_python as obs
import subprocess
import time
import os
from pathlib import Path

dir_atual = Path(__file__).resolve().parent

PORTA = 4455
HOST = 'localhost'
CMD_ABRIR_OBS = ["flatpak", "run", "com.obsproject.Studio"]

# Caminhos para sons do sistema Linux (comuns no Ubuntu/Fedora/Mint)
SOUND_SFX = str(dir_atual / "sfx" / "notify_sound.mp3")

def tocar_som(arquivo):
    """Toca um efeito sonoro usando o player nativo do sistema."""
    if os.path.exists(arquivo):
        subprocess.Popen(["ffplay", "-nodisp", "-autoexit", arquivo], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def esta_rodando():
    resultado = subprocess.run(["pgrep", "-x", "obs"], capture_output=True, text=True)
    return resultado.stdout != ""

def toggle_obs():
    if not esta_rodando():
        print("Abrindo OBS...")
        subprocess.Popen(CMD_ABRIR_OBS, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(1) 
        
        try:
            cl = obs.ReqClient(host=HOST, port=PORTA)
            cl.start_record()
            tocar_som(SOUND_SFX) # Efeito de início
        except Exception as e:
            print(f"Erro ao iniciar gravação: {e}")
    else:
        try:
            cl = obs.ReqClient(host=HOST, port=PORTA)
            status = cl.get_record_status()
            
            if status.output_active:
                cl.stop_record()
                tocar_som(SOUND_SFX) # Efeito de fim
                time.sleep(3)
            print("Fechando OBS...")
            subprocess.run(["pkill", "-f", "obs"])
        except Exception as e:
            print(f"Erro ao conectar/parar: {e}")

if __name__ == "__main__":
    toggle_obs()
