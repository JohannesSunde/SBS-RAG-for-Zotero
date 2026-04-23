from fastapi import APIRouter
from fastapi.responses import JSONResponse
import os
import signal
import threading
import time
import datetime
from backend.deps import get_profile_manager, DEFAULT_DB_PATH

router = APIRouter()

@router.get("/")
def read_root():
    """Health check endpoint for Zotero LLM backend."""
    return {"msg": "Welcome to Zotero LLM Plugin backend"}

@router.head("/")
def read_root_head():
    """Health check endpoint (HEAD) for Zotero LLM backend."""
    return {"msg": "Welcome to Zotero LLM Plugin backend"}

@router.get("/health")
def health_check_simple():
    """Simple health check endpoint for application startup verification."""
    return {"status": "healthy"}

@router.get("/api/health")
def health_check():
    """
    Detailed health check endpoint that validates all critical components.
    Returns structured health information for diagnostics.
    """
    profile_manager = get_profile_manager()
    try:
        health_status = {
            "status": "healthy",
            "timestamp": datetime.datetime.now().isoformat(),
            "components": {}
        }
        
        # Check profile system
        try:
            active = profile_manager.get_active_profile()
            health_status["components"]["profile_manager"] = {
                "status": "ok",
                "active_profile": active["id"] if active else None
            }
        except Exception as e:
            health_status["components"]["profile_manager"] = {
                "status": "error",
                "error": str(e)
            }
            health_status["status"] = "degraded"
        
        # Check database path
        try:
            # Note: DB_PATH depends on settings which we'll handle better later
            # For now just use the default or look it up
            db_exists = os.path.exists(DEFAULT_DB_PATH) # Simplified for health check
            health_status["components"]["database"] = {
                "status": "ok" if db_exists else "warning",
                "path": DEFAULT_DB_PATH,
                "exists": db_exists
            }
            if not db_exists:
                health_status["status"] = "degraded"
        except Exception as e:
            health_status["components"]["database"] = {
                "status": "error",
                "error": str(e)
            }
            health_status["status"] = "degraded"
        
        return health_status
        
    except Exception as e:
        return {
            "status": "unhealthy",
            "error": str(e),
            "timestamp": datetime.datetime.now().isoformat()
        }

@router.post("/shutdown")
async def shutdown():
    """
    Graceful shutdown endpoint.
    Allows Electron to cleanly shut down the backend server.
    """
    def delayed_shutdown():
        """Shutdown after a brief delay to allow response to be sent"""
        time.sleep(2.0)
        os.kill(os.getpid(), signal.SIGTERM)
    
    # Start shutdown in background thread
    thread = threading.Thread(target=delayed_shutdown, daemon=True)
    thread.start()
    
    return {"status": "shutting_down", "message": "Server will shutdown in 2 seconds"}
