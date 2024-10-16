from controller import Robot

TIME_STEP = 64 # Interval waktu dalam simulasi
MAX_SPEED = 6.28 # Kecepatan maksimum motor

# Inisialisasi robot
robot = Robot()

# Mendapatkan referensi motor kiri dan kanan
leftMotor = robot.getDevice('left wheel motor')
rightMotor = robot.getDevice('right wheel motor')

# Menyetel posisi motor agar bisa berputar tanpa batas
leftMotor.setPosition(float('inf'))
rightMotor.setPosition(float('inf'))

# Menyetel kecepatan motor agar bergerak maju
leftMotor.setVelocity(MAX_SPEED)
rightMotor.setVelocity(MAX_SPEED)

# Loop utama simulasi
while robot.step(TIME_STEP) != -1:
    pass # Robot akan terus bergerak maju 