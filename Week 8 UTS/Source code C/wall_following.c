/*
 * File:          wall_following.c
 * Date:
 * Description:
 * Author:     Ketut Satria Wibisana (1103213148)
 * Modifications:
 */

/*
 * You may need to add include files like <webots/distance_sensor.h> or
 * <webots/motor.h>, etc.
 */
#include <webots/robot.h>
#include <webots/motor.h>
#include <webots/distance_sensor.h>

//lib yang digunakan untuk input-output
#include <stdio.h>

/*
 * You may want to add macros here.
 */
 //deklarasi variabel global
#define TIME_STEP 64
//4.0 adalah kecepatan maksimal untuk e-puck
#define MAX_SPEED 4.0

/*
 * This is the main program.
 * The arguments of the main function can be specified by the
 * "controllerArgs" field of the Robot node
 */
int main(int argc, char **argv) {
  /* necessary to initialize webots stuff */
  wb_robot_init();

  /*
   * You should declare here WbDeviceTag variables for storing
   * robot devices like this:
   *  WbDeviceTag my_sensor = wb_robot_get_device("my_sensor");
   *  WbDeviceTag my_actuator = wb_robot_get_device("my_actuator");
   */

  // inisialisasi motor untuk robot e-puck
  WbDeviceTag left_motor = wb_robot_get_device("left wheel motor");
  WbDeviceTag right_motor = wb_robot_get_device("right wheel motor");
  
   /*
   *fungsi wb_motor_set_position digunakan untuk
   menetapkan/menyimpan posisi target yang baru untuk motor rotasi
   *INFINITY digunakan untuk gerakan rotasi tanpa akhir
   *fungsi wb_motor_set_velocity dapat digunakan untuk mengontrol 
   kecepatan putaran tanpa akhir
   */
   wb_motor_set_position(left_motor, INFINITY);
   wb_motor_set_position(right_motor, INFINITY);
   wb_motor_set_velocity(left_motor, 0.0);
   wb_motor_set_velocity(right_motor, 0.0);
   
   //membuat array untuk menyimpan semua proximity sensor (sensor jarak)
   // karena e-puck mempunyai total 8 buah prox_sensor mulai dari Ps0-Ps7
   WbDeviceTag prox_sensor[8];
   char prox_sensor_name[50];
   for (int ind = 0; ind < 8; ++ind) //membuat looping  untuk melewati semua sensor
   {
     //melakukan compute (menghitung) nama sensor dengan perintah printf
     sprintf(prox_sensor_name, "ps%d", ind);
     //Lalu menggunakan wb_robot_get_device  untuk mendapatkan sensor-sensornya
     prox_sensor[ind] = wb_robot_get_device(prox_sensor_name);
     //melakukan enable pada prox_sensor dengan menggunakan waktu
     wb_distance_sensor_enable(prox_sensor[ind], TIME_STEP);
    }
    
    /* main loop
    *Perform simulation steps of TIME_STEP miliseconds
    *and leave the loop when the simulation is over
    */
    // deklarasi variabel untuk mengontrol kemudi kiri dan kanan
    double left_speed = MAX_SPEED;
    double right_speed = MAX_SPEED;
    
  while (wb_robot_step(TIME_STEP) != -1) {
    /*
     * Read the sensors :
     * Enter here functions to read sensor data, like:
     *  double val = wb_distance_sensor_get_value(my_sensor);
     */

    //wb_distance_sensor_get_value digunakan untuk mendapatkan nilai sensor
    bool left_wall = wb_distance_sensor_get_value(prox_sensor[5]) > 80;
    //left_corner memastikan agar robot tidak terlalu dekat dengan dinding
    bool left_corner = wb_distance_sensor_get_value(prox_sensor[6]) > 80;
    bool front_wall = wb_distance_sensor_get_value(prox_sensor[7]) > 80;
    
    /* Process sensor data here */

    // jika robot mendeteksi adanya wall di depan
    // maka robot akan berbelok ke kiri dengan mengikuti wall sebelah kirinya
    if (front_wall == true)
    {
      left_speed = MAX_SPEED;
      right_speed = -MAX_SPEED;
    }
    else
    {
      // jika dinding terdeteksi pada sebelah kiri robot
      // maka robot akan terus berjalan lurus kedepan
      if (left_wall == true)
      {
        left_speed = MAX_SPEED;
        right_speed = MAX_SPEED;
      }
      
      // jika tidak ada dinding disekitar,
      // robot akan belok kiri (seperti berputar) untuk menemunkan tembok
      else
      {
        left_speed = MAX_SPEED/8;
        right_speed = MAX_SPEED;
      }
      
      //kondisi agar robot tidak terlalu mepet dengan dinding (Wall)
      if (left_corner == true)
      {
        left_speed = MAX_SPEED;
        right_speed = MAX_SPEED/8;
      }
     }
     
    /*
     * Enter here functions to send actuator commands, like:
     * wb_motor_set_position(my_actuator, 10.0);
     */
     
     /*
     memberikan kecepatan pada akuator,
     dimana aktuatornya adalah motor-motor pada robot e-puck
     sebagai komudi
     */
     wb_motor_set_velocity(left_motor, left_speed);
     wb_motor_set_velocity(right_motor, right_speed);
  };

  /* Enter your cleanup code here */

  /* This is necessary to cleanup webots resources */
  wb_robot_cleanup();

  return 0;
}
